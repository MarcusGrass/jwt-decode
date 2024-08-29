use crate::jwt::{jwt_is_valid, JwtPayload};
use crate::read_line::stdin_readline;
use crate::Args;
use alloc::format;
use alloc::string::{String, ToString};
use tiny_std::time::SystemTime;
use tiny_std::{println, UnixStr};

pub fn run(args: Args) -> Result<(), String> {
    let input = match (args.stdin, args.path) {
        (Some(_), Some(_)) => {
            return Err("Supplied both stdin and a path to read from".to_string());
        }
        (Some(input), None) => input,
        (None, Some(path)) => match tiny_std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(e) => {
                return Err(format!(
                    "Failed to read jwt from supplied path: {path:?}, e: {e}"
                ))
            }
        },
        (None, None) => stdin_readline()?,
    };
    let output = args.output.map(Output::parse).transpose()?;
    decode_no_sig_check(input.as_str(), args.verbose, args.readable_time, output)
}

fn decode_no_sig_check(
    input: &str,
    verbose: bool,
    fmt_time: bool,
    output: Option<Output>,
) -> Result<(), String> {
    let validated = jwt_is_valid(input.trim())?;
    if verbose {
        let pretty = match serde_json::to_string_pretty(&validated.header) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Failed to prettify json to print jwt header: {e}"));
            }
        };
        println!("Header {pretty}",);
    }
    if verbose {
        let pretty = fmt_payload(&validated.payload, fmt_time)?;
        println!("Payload {pretty}");
    }

    if let Some(selection) = output {
        match selection {
            Output::Payload => {
                let pretty = fmt_payload(&validated.payload, false)?;
                println!("{pretty}");
            }
            Output::Header => {
                let pretty = match serde_json::to_string_pretty(&validated.header) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(format!("Failed to prettify json to print jwt header: {e}"));
                    }
                };
                println!("{pretty}");
            }
        }
    }
    Ok(())
}

fn fmt_payload(jwt_payload: &JwtPayload, fmt_time: bool) -> Result<String, String> {
    match jwt_payload {
        JwtPayload::Json(j) => {
            if fmt_time {
                format_time(j)
            } else {
                serde_json::to_string_pretty(j)
                    .map_err(|_e| "Failed to pretty print json payload".to_string())
            }
        }
        JwtPayload::Utf8(s) => Ok(s.clone()),
        JwtPayload::Raw(_) => Ok("[RAW BYTES]".to_string()),
    }
}

fn format_time(json: &serde_json::Value) -> Result<String, String> {
    let mut to_modify = json.clone();
    fmt_if_present(&mut to_modify, "iat")?;
    fmt_if_present(&mut to_modify, "exp")?;
    fmt_if_present(&mut to_modify, "nbf")?;
    serde_json::to_string_pretty(&to_modify)
        .map_err(|_e| format!("Failed to pretty print json='{to_modify}'"))
}

fn fmt_if_present(json: &mut serde_json::Value, path: &'static str) -> Result<(), String> {
    let Some(val) = json.get_mut(path) else {
        return Ok(());
    };
    let Some(seconds_since_epoch) = val.as_i64() else {
        return Ok(());
    };
    let now = SystemTime::now().duration_since_unix_time().as_secs() as i64;
    let until = seconds_since_epoch - now;
    let odt = time::OffsetDateTime::from_unix_timestamp(seconds_since_epoch).map_err(|_e| {
        format!("Failed to convert field at {path} ({seconds_since_epoch}) to an offset datetime")
    })?;
    if until >= 0 {
        *val = serde_json::Value::String(format!(
            "{seconds_since_epoch} [{odt} (In {until} seconds)]"
        ));
    } else {
        let before = -until;
        *val = serde_json::Value::String(format!(
            "{seconds_since_epoch} [{odt} ({before} seconds ago)]"
        ));
    }
    Ok(())
}

#[derive(Debug)]
enum Output {
    Payload,
    Header,
}

impl Output {
    fn parse(input: &'static UnixStr) -> Result<Self, String> {
        match input
            .as_str()
            .map_err(|e| format!("Specified output format was not valid utf8: {e}"))?
        {
            "header" => Ok(Self::Header),
            "payload" => Ok(Self::Payload),
            unk => Err(format!("Unknown output selection: {unk}")),
        }
    }
}
