#![no_std]
extern crate alloc;

use alloc::string::String;
use tiny_cli::ArgParse;
use tiny_std::unix::cli::parse_cli_args;
use tiny_std::{eprintln, UnixStr};

mod jwt;
mod read_line;
pub mod run;

#[derive(Debug, ArgParse)]
pub struct Args {
    /// Pass the token as an argument, mandatory if not using `-p, --path`, should be
    /// a single line, mutually exclusive with `-p, --path`.
    pub stdin: Option<String>,

    /// Supply the token through a file, mandatory if not passing token using `s, --stdin`.
    /// The first line of the file should contain the token, mutually exclusive with STDIN.
    #[cli(short = "p", long = "path")]
    pub path: Option<&'static UnixStr>,

    /// Print debug output, like the raw header component etc.
    #[cli(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Show readable time in verbose output.
    /// Will display difference between common payload fields `iat`, `exp`, and `nbf`
    /// and the current system time. Only used if `--verbose` is specified
    #[cli(short = "r", long = "readable-time")]
    pub readable_time: bool,

    /// Select what's output to stdout, overrides verbose and quiet
    /// Accepted values:
    /// 1. `payload` for outputting the jwt claims as json
    /// 2. `header` for outputting the jwt header as json
    #[cli(short = "o", long = "output")]
    pub output: Option<&'static UnixStr>,
}

pub fn run() -> i32 {
    let args = parse_cli_args::<Args>();
    if let Err(e) = run::run(args) {
        eprintln!("Failed to run command: {e}");
        return 1;
    }
    0
}
