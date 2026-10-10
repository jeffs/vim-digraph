# Vim Digraph

A CLI and library to ease entry of Unicode characters.

The `vd` CLI and library can expand all Vim digraphs, as well as literal
codepoint values (specified as hexadecimal) and mnemonic aliases (such as
"thumbsup" for `👍`). This is useful for extending other editors to support
digraphs by shelling out to the `vd` executable. For example, in Helix  you
might run `!vd n?` to insert `ñ`, or `!vd shift cmd Q` for `⇧⌘Q`.

This crate has no external dependencies.

## Command line usage

```sh
# Digraphs; e.g., for EM DASH and UPWARDS ARROW
vd -M -v         # —↓

# Codepoints
vd 1f600 1f918   # 😀🤘 
vd a9            # ©

# Aliases
vd frog poop lol # 🐸💩😂
vd command shift # ⌘⇧
```

## License

All Rust code in this repository is licensed under [Apache 2.0][], but note
that the repo also includes a copy of Vim's [`digraph.txt`][digraph.txt]
reference. (A build script reads the digraphs to generate equivalent Rust
source code.) That one text file is distributed under [OPUBL 1.0]; see
[`vim/README.md`][vim/README.md] for details.

[Apache 2.0]: https://spdx.org/licenses/Apache-2.0.html
[OPUBL 1.0]: https://spdx.org/licenses/OPUBL-1.0.html
[digraph.txt]: ./vim/digraph.txt
[vim/README.md]: ./vim/README.md
