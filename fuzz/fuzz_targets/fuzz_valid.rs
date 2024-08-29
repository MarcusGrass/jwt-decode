#![no_main]

use base64::Engine;
use decode_lib::Args;
use libfuzzer_sys::arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

#[derive(Debug)]
pub struct ValidJwt {
    inner: String,
}

impl<'a> Arbitrary<'a> for ValidJwt {
    fn arbitrary(u: &mut Unstructured<'a>) -> libfuzzer_sys::arbitrary::Result<Self> {
        let header = b"{
\"alg\": \"HS256\",
\"typ\": \"JWT\"
}";
        let body: &[u8] = u.arbitrary()?;
        let sig: &[u8] = u.arbitrary()?;
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header);
        let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(body);
        let sig = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(sig);
        let jwt = format!("{header}.{body}.{sig}");
        Ok(ValidJwt { inner: jwt })
    }
}

fuzz_target!(|data: ValidJwt| {
    let args = Args {
        stdin: Some(data.inner.clone()),
        path: None,
        verbose: false,
        readable_time: false,
        output: None,
    };
    // Odds of getting a valid JWT is really low
    assert!(
        decode_lib::run::run(args).is_ok(),
        "Got a error code from input={}",
        data.inner
    );
});
