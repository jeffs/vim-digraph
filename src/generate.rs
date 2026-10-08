//! Code generation, for transpiling `digraph.txt` to Rust source code.

use std::{fs, io, path::Path};

/// Maps a two-byte input sequence to a Unicode character.
#[derive(Debug)]
pub struct Digraph {
    /// ASCII characters.
    pub input: [u8; 2],
    /// Unicode character represented by the input.
    pub output: char,
}

/// Parses the table of digraphs from a `digraph.txt` file's contents.
///
/// The input format is observed from Vim's actual `digraph.txt`: The table has
/// five tab-separated columns, and the second column always has two characters
/// (hence "digraph"). There are other five-column lines, but they don't have a
/// two-byte sequence in the second column.
fn collect_rows(text: &str) -> Vec<Digraph> {
    text.lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter_map(|row| <[_; 5]>::try_from(row).ok())
        .filter_map(|[_char, digraph, _hex, dec, _official_name]| {
            Some(Digraph {
                input: digraph.as_bytes().try_into().ok()?,
                output: dec.parse().ok().and_then(char::from_u32)?,
            })
        })
        .collect()
}

/// Transpiles digraphs from Vim's reference manual format to Rust source code.
///
/// Writes the source code to the specified `.rs` file. The resulting source
/// code defines a table expression, sorted by [`Digraph::input`] so users can
/// look up digraphs using binary search. See also this project's build script
/// and `vim` directory.
///
/// # Errors
///
/// Returns an error if the input file cannot be read.
pub fn digraphs(digraph_txt: &Path, table_rs: &Path) -> io::Result<()> {
    let text = fs::read_to_string(digraph_txt)?;
    let mut rows = collect_rows(&text);
    rows.sort_unstable_by_key(|d| d.input);
    fs::write(table_rs, format!("{rows:#?}"))
}
