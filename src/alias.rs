//! Character lookup by named alias; e.g., "horns" for '🤘'.

/// Association list of alias names to characters.
pub const ALIASES: &[(&str, char)] = &[
    ("apostrophe", '’'), // RIGHT SINGLE QUOTATION MARK
    ("backspace", '⌫'),  // NB: Apple calls backspace Delete.
    ("cent", '¢'),       // CENT SIGN
    ("cents", '¢'),      // CENT SIGN
    ("cmd", '⌘'),        // PLACE OF INTEREST SIGN
    ("command", '⌘'),    // PLACE OF INTEREST SIGN
    ("control", '⌃'),    // UP ARROWHEAD
    ("ctrl", '⌃'),       // UP ARROWHEAD
    ("del", '⌦'),        // NB: Apple calls delete Forward Delete.
    ("delete", '⌦'),     //
    ("down", '↓'),       // DOWNWARDS ARROW
    ("emdash", '—'),     // EM DASH
    ("endash", '–'),     // EN DASH
    ("facepalm", '🤦'),  //
    ("frog", '🐸'),      //
    ("grimace", '😬'),   //
    ("horns", '🤘'),     //
    ("ldq", '“'),        // LEFT SINGLE QUOTATION MARK
    ("left", '←'),       //
    ("lol", '😂'),       //
    ("lsq", '‘'),        // LEFT SINGLE QUOTATION MARK
    ("ok", '👌'),        //
    ("okhand", '👌'),    //
    ("opt", '⌥'),        // OPTION KEY
    ("option", '⌥'),     // OPTION KEY
    ("poop", '💩'),      //
    ("rdq", '”'),        // RIGHT DOUBLE QUOTATION MARK
    ("right", '→'),      // RIGHTWARDS ARROW
    ("rsq", '’'),        // RIGHT SINGLE QUOTATION MARK
    ("shift", '⇧'),      // UPWARDS WHITE ARROW
    ("sob", '😭'),       //
    ("up", '↑'),         // UPWARDS ARROW
];

/// Maps memorable names to common symbols.
#[must_use]
pub fn parse(arg: &str) -> Option<char> {
    ALIASES
        .binary_search_by_key(&arg, |a| a.0)
        .ok()
        .map(|i| ALIASES[i].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases_are_sorted_by_key() {
        assert!(ALIASES.is_sorted_by_key(|(k, _v)| *k));
    }

    #[test]
    fn aliases_do_not_collide_with_digraphs() {
        for key in ALIASES.iter().map(|(k, _)| *k) {
            let Ok(pair) = key.as_bytes().try_into() else {
                continue; // Wrong length for a digraph.
            };
            assert_eq!(crate::digraph::find_char(pair), None, "{key}");
        }
    }
}
