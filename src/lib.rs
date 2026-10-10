//! [digraph.txt]: https://github.com/jeffs/vim-digraph/blob/main/vim/digraph.txt
//! [vim/README.md]: https://github.com/jeffs/vim-digraph/blob/main/vim/README.md
#![doc = include_str!("../README.md")]

// # TODO
//
// - [ ] Support [emoticons](https://gist.github.com/jordanorelli/11229304)
// - [ ] Support standard aliases for all emojis.
// - [ ] Support both "juicy" and dry variants; e.g., 🐸 vs 𓆏, or ♟️ vs ♟
// - [ ] Support skin tones
// - [ ] Figure out why Wezterm doesn't support [Egyptian hieroglyphs][].
//
// [Egyptian hieroglyphs]: https://unicode-explorer.com/b/13000

pub mod alias;
pub mod digraph;
pub mod generate;

/// Parses a digraph, codepoint, or alias.
///
/// If arg is a single byte, it is returned directly.
pub fn parse(arg: &str) -> Option<char> {
    if let &[c] = arg.as_bytes() {
        Some(char::from(c))
    } else if let Some(c) = arg.as_bytes().try_into().ok().and_then(digraph::find_char) {
        Some(c)
    } else if let Some(c) = u32::from_str_radix(arg, 16).ok().and_then(char::from_u32) {
        Some(c)
    } else {
        alias::parse(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preserves_single_ascii_characters() {
        let buf = &mut [0; 4];
        for c in '\0'..='\x7f' {
            assert_eq!(parse(c.encode_utf8(buf)), Some(c));
        }
    }

    #[test]
    fn parse_expands_recognized_symbols() {
        assert_eq!(parse("OK"), Some('✓')); // digraph
        assert_eq!(parse("1fa77"), Some('🩷')); // codepoint
        assert_eq!(parse("ok"), Some('👌')); // alias
        assert_eq!(parse("ZZ"), None);
    }
}
