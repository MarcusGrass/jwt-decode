use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rusl::platform::STDIN;

pub(crate) fn stdin_readline() -> Result<String, String> {
    let mut s = Vec::new();
    let mut buf = [0u8; 256];
    let mut has_newline = false;
    loop {
        let bytes_read = rusl::unistd::read(STDIN, &mut buf)
            .map_err(|e| format!("Failed to read from stdin into buf: {e}"))?;
        if bytes_read == 0 {
            return Err("Unexpected EOF waiting for stdin".to_string());
        }
        for (offset, byte) in buf[..bytes_read].iter().enumerate() {
            if *byte == b'\n' {
                s.extend_from_slice(&buf[..offset]);
                has_newline = true;
                break;
            }
        }
        if has_newline {
            break;
        }
        s.extend_from_slice(&buf);
    }
    String::from_utf8(s)
        .map_err(|_| "Read up until newline from stdin, the bytes read was not utf-8".to_string())
}
