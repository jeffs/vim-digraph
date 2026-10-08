//! Digraph lookup per Vim's [`**digraph-table**`]; e.g., "OK" for '✓'.
//!
//! [`**digraph-table**`]: https://github.com/vim/vim/blob/b2b47bdf9401c865780dd4c47b1dbafb0463c75a/runtime/doc/digraph.txt#L184

// Re-export so struct Digraph is defined in mod digraph, as one might expect.
// The definition is actually in the code generator so that the same structure
// is used for both writing and reading the generated table of digraphs.
pub use crate::generate::Digraph;

/// The complete set of recognized digraphs.
///
/// Generated from [the Vim reference manual] by this project's build script.
/// See also [`crate::generate::digraphs`].
///
/// [the Vim reference manual]: https://github.com/vim/vim/blob/master/runtime/doc/digraph.txt
///
/// # Note
///
/// Normalization to NFC would corrupt this table. The only non-NFC entries
/// are OHM SIGN (U+2126) and ANGSTROM SIGN (U+212B); NFC maps those to GREEK
/// CAPITAL LETTER OMEGA (U+03A9) and LATIN CAPITAL LETTER A WITH RING ABOVE
/// (U+00C5). If we normalized to NFC, the Om and AO digraphs would thus produce
/// different characters from Vim.
#[expect(clippy::unicode_not_nfc)]
pub const TABLE: &[Digraph] = &include!(concat!(env!("OUT_DIR"), "/digraph-table.rs"));

/// Looks up the character represented by the digraph.
#[must_use]
pub fn find_char(digraph: [u8; 2]) -> Option<char> {
    TABLE
        .binary_search_by_key(&digraph, |Digraph { input: digraph, .. }| *digraph)
        .ok()
        .map(|i| TABLE[i].output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_sorted_by_key() {
        assert!(TABLE.is_sorted_by_key(|Digraph { input, .. }| input));
    }

    /// Checks a few common digraphs supported by Vim.
    #[test]
    fn find_char_works() {
        assert_eq!(find_char(*b"-M"), Some('—'));
        assert_eq!(find_char(*b"-N"), Some('–'));
        assert_eq!(find_char(*b"'6"), Some('‘'));
        assert_eq!(find_char(*b"'9"), Some('’'));
        assert_eq!(find_char(*b"\"6"), Some('“'));
        assert_eq!(find_char(*b"\"9"), Some('”'));
        assert_eq!(find_char(*b"XY"), None);
    }
}
