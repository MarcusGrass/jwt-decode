#![no_main]

use decode_lib::Args;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let args = Args {
        stdin: Some(data.to_string()),
        path: None,
        verbose: false,
        readable_time: false,
        output: None,
    };
    // Odds of getting a valid JWT is really low
    assert!(
        decode_lib::run::run(args).is_err(),
        "Got a success code from input={data}"
    );
});
