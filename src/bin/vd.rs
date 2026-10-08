//! CLI for mapping digraphs, codepoints, and aliaes to Unicode characters.
//!
//! See also the accompanying `README.md`.

// # TODO
//
// * [] Support [emoticons](https://gist.github.com/jordanorelli/11229304)
// * [] Support both "juicy" and dry variants; e.g., 🐸 vs 𓆏, or ♟️ vs ♟
// * [] Figure out why Wezterm doesn't support [Egyptian hieroglyphs].
//
// [Egyptian hieroglyphs]: https://unicode-explorer.com/b/13000

use std::{env, process::ExitCode};

use vim_digraph as vd;

/// Attempts to parse the arg as a digraph, codepoint, or alias, in that order.
fn parse_arg(arg: &str) -> Option<char> {
    if let Some(char) = arg
        .as_bytes()
        .try_into()
        .ok()
        .and_then(vd::digraph::find_char)
    {
        Some(char)
    } else if let Some(char) = u32::from_str_radix(arg, 16).ok().and_then(char::from_u32) {
        Some(char)
    } else {
        vd::alias::parse(arg)
    }
}

/// Attempts to parse all arguments to the program.
///
/// See also [`parse_arg`].
///
/// # Errors
///
/// Returns the first unparseable arg, if any, as an error.
fn parse_args() -> Result<Vec<char>, String> {
    env::args()
        .skip(1)
        .map(|arg| parse_arg(&arg).ok_or(arg))
        .collect()
}

fn main() -> ExitCode {
    let chars = parse_args();
    match chars {
        Ok(chars) if chars.is_empty() => {
            eprintln!("usage: vd <DIGRAPH | CODEPOINT | ALIAS>...");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_arg_works() {
        assert_eq!(parse_arg("OK"), Some('✓')); // digraph
        assert_eq!(parse_arg("1fa77"), Some('🩷')); // codepoint
        assert_eq!(parse_arg("ok"), Some('👌')); // alias
        assert_eq!(parse_arg("ZZ"), None);
    }
}
