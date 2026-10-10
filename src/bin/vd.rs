#![doc = include_str!("../../README.md")]

use std::{env, process::ExitCode};

use vim_digraph as vd;

const USAGE: &str = "usage: vd <DIGRAPH| CODEPOINT | ALIAS>...";

/// Parses all arguments to this program.
///
/// See also [`vd::parse`].
///
/// # Errors
///
/// Returns the first unparseable arg, if any, as an error.
fn parse_args() -> Result<Vec<char>, String> {
    env::args()
        .skip(1)
        .map(|arg| vd::parse(&arg).ok_or(arg))
        .collect()
}

fn main() -> ExitCode {
    // Fortunately, "-h" is not a digraph.
    if env::args().any(|s| s == "-h" || s == "--help") {
        eprintln!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    match parse_args() {
        Ok(chars) if chars.is_empty() => {
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
        Ok(chars) => {
            for c in chars {
                print!("{c}");
            }
            println!();
            ExitCode::SUCCESS
        }
        Err(arg) => {
            eprintln!("error: bad arg: {arg}");
            ExitCode::FAILURE
        }
    }
}
