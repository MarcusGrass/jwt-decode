use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use base64::Engine;

pub struct ValidatedOutput {
    pub header: serde_json::Value,
    pub payload: JwtPayload,
    pub _signature: Vec<u8>,
}

pub enum JwtPayload {
    Json(serde_json::Value),
    Utf8(String),
    #[allow(unused)]
    Raw(Vec<u8>),
}

/// [Direct instructions from the RFC](https://datatracker.ietf.org/doc/html/rfc7519#section-7.2)
pub fn jwt_is_valid(raw: &str) -> Result<ValidatedOutput, String> {
    // 1. Contains at least one period
    // 2. Header is the portion before the first period
    let (header, rest) = raw
        .split_once('.')
        .ok_or_else(|| format!("Raw JWT input did not contain at least one dot (.): {raw}"))?;
    // 3. Base64url decode the header
    // 4a. Verify that the header is utf8
    let decoded_header = try_decode_string_value_b64(header)?;
    // 4b. Verify that the header is a valid json object
    let header_deserialized: serde_json::Value =
        serde_json::from_str(&decoded_header).map_err(|_e| {
            format!(
                "Failed to deserialize header, not a valid JSON object: {}",
                raw
            )
        })?;
    // 5. Verify that the JOSE header contains valid parameters
    validate_from_header(header_deserialized, rest)
}

fn try_decode_string_value_b64(input: &str) -> Result<String, String> {
    let raw = try_decode_b64(input)?;
    String::from_utf8(raw).map_err(|_e| format!("The decoded base64 was not valid utf8: '{input}'"))
}

// *Should* as far as I can tell be URL_SAFE_NO_PAD
fn try_decode_b64(input: &str) -> Result<Vec<u8>, String> {
    match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(format!(
            "Failed to decode JWT component as url-safe b64 without padding, e: {e}"
        )),
    }
}

/// [RFC-7515](https://datatracker.ietf.org/doc/html/rfc7515#section-4)
fn validate_from_header(raw: serde_json::Value, rest: &str) -> Result<ValidatedOutput, String> {
    raw.as_object()
        .ok_or_else(|| format!("Raw json was not a valid object: '{raw}'"))?;
    assert_ascii_at(&raw, "alg")?;
    let Some(_enc) = raw.get("enc") else {
        let (body, sig) = rest.split_once('.')
            .ok_or_else(|| format!("Interpreted input as a JWS, but the input did not contain 2 dots, rest of input='{rest}'"))?;
        return validate_jws(raw, body, sig);
    };
    Err(format!("Got a header that suggests that the supplied JWT is a JWE, this tool only handles JWS, header={raw}"))
}

fn assert_ascii_at(header: &serde_json::Value, path: &'static str) -> Result<(), String> {
    let content = header
        .get(path)
        .ok_or_else(|| format!("Header did not contain a '{path}'-field: '{header}'"))?;
    let maybe_ascii = content.as_str().ok_or_else(|| {
        format!("The provided header's {path}-field did not contain valid ASCII: '{content}'")
    })?;
    // Specified it has to be ascii
    if !maybe_ascii.chars().all(|ch| ch.is_ascii()) {
        return Err(format!(
            "The provided header's {path}-field did not contain valid ASCII: '{maybe_ascii}'"
        ));
    }
    Ok(())
}

/// [RFC-7515](https://datatracker.ietf.org/doc/html/rfc7515)
/// [Section for validation](https://datatracker.ietf.org/doc/html/rfc7515#section-3.3)
fn validate_jws(
    header: serde_json::Value,
    body: &str,
    signature: &str,
) -> Result<ValidatedOutput, String> {
    // The header is already validated
    // Aside from validating the signature, which is out of scope, the body and signature just
    // needs to be valid b64
    let body_raw = try_decode_b64(body)?;
    let sig_raw = try_decode_b64(signature)?;
    if let Ok(utf8) = core::str::from_utf8(&body_raw) {
        if let Ok(json) = serde_json::from_str(utf8) {
            Ok(ValidatedOutput {
                header,
                payload: JwtPayload::Json(json),
                _signature: sig_raw,
            })
        } else {
            Ok(ValidatedOutput {
                header,
                payload: JwtPayload::Utf8(utf8.to_string()),
                _signature: sig_raw,
            })
        }
    } else {
        Ok(ValidatedOutput {
            header,
            payload: JwtPayload::Raw(body_raw),
            _signature: sig_raw,
        })
    }
}
