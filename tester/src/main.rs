use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
const RAW_BYTES_TOKEN: &str = "[RAW BYTES]\n";

const VALID_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI3Nzc3NyIsIm5hbWUiOiJNeSB0ZXN0IG5hbWUiLCJpYXQiOjE1MTYyMzkwMzl9.r0YSbk-Gjr4gWATqbDnirs102IUBQRru-_TNu5AtE18";

const VALID_TOKEN_HEADER: &[u8] = b"\
{
  \"alg\": \"HS256\",
  \"typ\": \"JWT\"
}
";
const VALID_TOKEN_PAYLOAD: &[u8] = b"\
{
  \"iat\": 1516239039,
  \"name\": \"My test name\",
  \"sub\": \"77777\"
}
";

// It's not a valid JWT (bad sig), but for the purposes of this tool it's fine.
const VALID_TOKEN_2: &str =
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.aGVsbG8K.r0YSbk-Gjr4gWATqbDnirs102IUBQRru-_TNu5AtE18";
const VALID_TOKEN_2_PAYLOAD: &[u8] = b"hello\n\n";

// This is not a valid JWT either, but for the purposes of this tool it's fine, the payload is head -n 1 /dev/urandom
const VALID_TOKEN_3: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\
.X3SCo836mgQJXDQ4sqd1Wgg-O0QuJrZpGS9NxtfNNyrpnK2t3vCxGRXqGwOAdDIYi7BMPiEt4RTg\
XzIzl8wQ5ZI-tlP5zPhrVTMdhgtmSG65YbTrqpQvXrLiCDJTKaHQZR4M213Pv8liMuxDR9kch133\
UPsLyMETR5Pc7czVyXh09Fmlix6OfFT02SdGS7CItN_bTjFgPBy7rGbf8Kfn2rU5XlgmmWbCFD_B\
5cGUmY-CREklFoMbd31IEuVAoZt2wN744a1lgFBVPPC5S_LPBR2de88DsBoiWMc2FQo\
.r0YSbk-Gjr4gWATqbDnirs102IUBQRru-_TNu5AtE18";

// const VALID_TOKEN_3_PAYLOAD: &[u8] = include_bytes!("../../.local/data/test3-bytes");

const VALID_TOKEN_4: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\
.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNzI0\
OTUyOTQwLCJuYmYiOjE3MjQ5NTI5NTAsImV4cCI6MTcyNDk5Mjk0MH0\
.ZrQSkD_tzm7yyTC0Cw3_qmBRZhi0QCirL4hfICAWNr0";

const VALID_TOKEN_4_PAYLOAD: &str = "\
{
  \"exp\": 1724992940,
  \"iat\": 1724952940,
  \"name\": \"John Doe\",
  \"nbf\": 1724952950,
  \"sub\": \"1234567890\"
}
";

fn main() {
    let mut args = std::env::args();
    args.next();
    let binary_to_test = args
        .next()
        .expect("Expected a single argument, the binary to test");
    for (token, payload, verbose) in [
        (VALID_TOKEN, VALID_TOKEN_PAYLOAD, verbose_output1()),
        (VALID_TOKEN_2, VALID_TOKEN_2_PAYLOAD, verbose_output2()),
        (VALID_TOKEN_3, RAW_BYTES_TOKEN.as_bytes(), verbose_output3()),
        (
            VALID_TOKEN_4,
            VALID_TOKEN_4_PAYLOAD.as_bytes(),
            verbose_output4(),
        ),
    ] {
        token_as_arg_no_out(&binary_to_test, token);
        token_as_arg_header_output(&binary_to_test, token, VALID_TOKEN_HEADER);
        token_as_arg_payload_out(&binary_to_test, token, payload);
        token_as_arg_verbose(&binary_to_test, token, verbose.as_bytes());

        stdin_no_out(&binary_to_test, token);
        stdin_header_out(&binary_to_test, token, VALID_TOKEN_HEADER);
        stdin_payload_out(&binary_to_test, token, payload);
        stdin_verbose(&binary_to_test, token, verbose.as_bytes());
    }

    let file_1 = Path::new("./.local/data/test.jwt");
    let file_2 = Path::new("./.local/data/test2.jwt");
    let file_3 = Path::new("./.local/data/test3.jwt");
    let file_4 = Path::new("./.local/data/test4.jwt");
    for (file, payload, verbose) in [
        (file_1, VALID_TOKEN_PAYLOAD, verbose_output1()),
        (file_2, VALID_TOKEN_2_PAYLOAD, verbose_output2()),
        (file_3, RAW_BYTES_TOKEN.as_bytes(), verbose_output3()),
        (file_4, VALID_TOKEN_4_PAYLOAD.as_bytes(), verbose_output4()),
    ] {
        from_file_no_out(&binary_to_test, file);
        from_file_output_header(&binary_to_test, file, VALID_TOKEN_HEADER);
        from_file_output_payload(&binary_to_test, file, payload);
        from_file_no_out_verbose(&binary_to_test, file, verbose.as_bytes());
    }
    time_does_noting_if_not_verbose(&binary_to_test);
    time_changes_output_if_verbose(&binary_to_test);
}

fn token_as_arg_no_out(bin: &str, token: &str) {
    let out = std::process::Command::new(bin)
        .arg(token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with valid token input");
    child_expect(out, ExpectOutput::Nothing, "arg_no_out");
    println!("[Success] Ran token as arg without output");
}

fn token_as_arg_header_output(bin: &str, token: &str, expect: &[u8]) {
    let out = std::process::Command::new(bin)
        .arg(token)
        .arg("-o")
        .arg("header")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with valid token input");
    child_expect(out, ExpectOutput::Stdout(expect), "arg_header_out");
    println!("[Success] Ran token as arg header output");
}

fn token_as_arg_payload_out(bin: &str, token: &str, expect: &[u8]) {
    let out = std::process::Command::new(bin)
        .arg(token)
        .arg("-o")
        .arg("payload")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with valid token input");
    child_expect(out, ExpectOutput::Stdout(expect), "arg_payload_out");
    println!("[Success] Ran token as arg payload output");
}

fn token_as_arg_verbose(bin: &str, token: &str, expect: &[u8]) {
    let out = std::process::Command::new(bin)
        .arg(token)
        .arg("-v")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with valid token input");
    child_expect(out, ExpectOutput::Stdout(expect), "arg_no_out_verbose");
    println!("[Success] Ran token as arg verbose");
}

fn from_file_no_out(bin: &str, file: &Path) {
    let out = std::process::Command::new(bin)
        .arg("-p")
        .arg(file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    child_expect(out, ExpectOutput::Nothing, "file_no_out");
    println!("[Success] Ran token from file without output");
}

fn from_file_output_header(bin: &str, file: &Path, expect: &[u8]) {
    let out = std::process::Command::new(bin)
        .arg("-p")
        .arg(file)
        .arg("-o")
        .arg("header")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    child_expect(out, ExpectOutput::Stdout(expect), "file_header_out");
    println!("[Success] Ran token from file header output");
}

fn from_file_output_payload(bin: &str, file: &Path, expect: &[u8]) {
    let out = std::process::Command::new(bin)
        .arg("-p")
        .arg(file)
        .arg("-o")
        .arg("payload")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    child_expect(out, ExpectOutput::Stdout(expect), "file_payload_out");
    println!("[Success] Ran token from file payload output");
}

fn from_file_no_out_verbose(bin: &str, file: &Path, expect: &[u8]) {
    let out = std::process::Command::new(bin)
        .arg("-p")
        .arg(file)
        .arg("-v")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    child_expect(out, ExpectOutput::Stdout(expect), "file_no_out_verbose");
    println!("[Success] Ran token from file verbose");
}

fn stdin_no_out(bin: &str, token: &str) {
    let mut out = std::process::Command::new(bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    out.stdin
        .as_mut()
        .unwrap()
        .write_fmt(format_args!("{token}\n"))
        .expect("Failed to write token to child stdin");
    child_expect(out, ExpectOutput::Nothing, "stdin_no_out");
    println!("[Success] Ran token from stdin without output");
}

fn stdin_header_out(bin: &str, token: &str, expect: &[u8]) {
    let mut out = std::process::Command::new(bin)
        .arg("-o")
        .arg("header")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    out.stdin
        .as_mut()
        .unwrap()
        .write_fmt(format_args!("{token}\n"))
        .expect("Failed to write token to child stdin");
    child_expect(out, ExpectOutput::Stdout(expect), "stdin_header_out");
    println!("[Success] Ran token from stdin with header output");
}

fn stdin_payload_out(bin: &str, token: &str, expect: &[u8]) {
    let mut out = std::process::Command::new(bin)
        .arg("-o")
        .arg("payload")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    out.stdin
        .as_mut()
        .unwrap()
        .write_fmt(format_args!("{token}\n"))
        .expect("Failed to write token to child stdin");
    child_expect(out, ExpectOutput::Stdout(expect), "stdin_payload_out");
    println!("[Success] Ran token from stdin with payload output");
}

fn stdin_verbose(bin: &str, token: &str, expect: &[u8]) {
    let mut out = std::process::Command::new(bin)
        .arg("-v")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run process with file input");
    out.stdin
        .as_mut()
        .unwrap()
        .write_fmt(format_args!("{token}\n"))
        .expect("Failed to write token to child stdin");
    child_expect(out, ExpectOutput::Stdout(expect), "stdin_verbose");
    println!("[Success] Ran token from stdin verbose");
}

fn verbose_output1() -> String {
    format!(
        "Header {}Payload {}",
        core::str::from_utf8(VALID_TOKEN_HEADER).unwrap(),
        core::str::from_utf8(VALID_TOKEN_PAYLOAD).unwrap()
    )
}

fn verbose_output2() -> String {
    format!(
        "Header {}Payload {}",
        core::str::from_utf8(VALID_TOKEN_HEADER).unwrap(),
        core::str::from_utf8(VALID_TOKEN_2_PAYLOAD).unwrap()
    )
}

fn verbose_output3() -> String {
    format!(
        "Header {}Payload {RAW_BYTES_TOKEN}",
        core::str::from_utf8(VALID_TOKEN_HEADER).unwrap()
    )
}

fn verbose_output4() -> String {
    format!(
        "Header {}Payload {}",
        core::str::from_utf8(VALID_TOKEN_HEADER).unwrap(),
        VALID_TOKEN_4_PAYLOAD
    )
}

pub enum ExpectOutput<'a> {
    Nothing,
    Stdout(&'a [u8]),
}

fn child_expect(child: Child, expect_output: ExpectOutput, label: &str) {
    let out = child
        .wait_with_output()
        .expect("Failed to await child exit");
    match expect_output {
        ExpectOutput::Nothing => {
            if !out.stdout.is_empty() {
                panic!(
                    "[Failure] {label} expected empty stdout, got {:?}",
                    core::str::from_utf8(&out.stdout)
                );
            }
            if !out.stderr.is_empty() {
                panic!(
                    "[Failure] {label} expected empty stderr, got {:?}",
                    core::str::from_utf8(&out.stderr)
                );
            }
            assert!(
                out.status.success(),
                "[Failure] {label} Non-success exit code"
            );
        }
        ExpectOutput::Stdout(wants) => {
            assert_eq!(
                wants,
                &out.stdout,
                "[Failure] {label} {:?} != {:?}",
                core::str::from_utf8(wants),
                core::str::from_utf8(&out.stdout)
            );
            if !out.stderr.is_empty() {
                panic!(
                    "[Failure] {label} expected empty stderr, got {:?}",
                    core::str::from_utf8(&out.stderr)
                );
            }
            assert!(
                out.status.success(),
                "[Failure] {label} Non-success exit code"
            );
        }
    }
}

pub fn time_does_noting_if_not_verbose(bin: &str) {
    let out_no_verbose = Command::new(bin)
        .arg(VALID_TOKEN_4)
        .output()
        .expect("Failed to run command");
    let out_no_verbose_time = Command::new(bin)
        .arg(VALID_TOKEN_4)
        .arg("-r")
        .output()
        .expect("Failed to run command");
    assert_eq!(
        out_no_verbose, out_no_verbose_time,
        "[Failed] different output with and without -r without -v"
    );
    println!("[Success] Ran -r with and without -v, producing the same results");
}

pub fn time_changes_output_if_verbose(bin: &str) {
    let out_no_verbose_time = Command::new(bin)
        .arg(VALID_TOKEN_4)
        .arg("-r")
        .output()
        .expect("Failed to run command");
    assert!(
        out_no_verbose_time.stderr.is_empty(),
        "[Failed] time_changes_output has stderr"
    );
    assert!(
        out_no_verbose_time.status.success(),
        "[Failed] time_changes_output got error code from cmd"
    );

    let out_verbose_time = Command::new(bin)
        .arg(VALID_TOKEN_4)
        .arg("-v")
        .arg("-r")
        .output()
        .expect("Failed to run command");
    assert!(
        out_verbose_time.stderr.is_empty(),
        "[Failed] time_changes_output has stderr"
    );
    assert!(
        out_verbose_time.status.success(),
        "[Failed] time_changes_output got error code from cmd"
    );
    // Lazy check
    assert!(
        out_verbose_time.stdout.len() > out_no_verbose_time.stdout.len(),
        "[Failure] time_changes_output output with time was not longer than output without time"
    );
    println!("[Success] Ran -r with -v, producing the longer results");
}
