# Vim Digraph

This package provides a library and a command-line tool for expanding Vim
digraphs or codepoints specified in hexadecimal to Unicode characters. A
few aliases (such as "down" for "↓") are also supported. This is useful
for extending other editors to support digraphs by shelling out to the `vd`
executable. For example, in Helix Normal mode you might run `!vd n?` to insert
`ñ`, or `!vd cmd` to insert the Command key symbol `⌘`.

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
that the repo also includes a copy of Vim's [`digraph.txt`](./vim/digraph.txt)
reference. (A build script reads the digraphs to generate equivalent Rust
source code.) That one text file is distributed under [OPUBL 1.0]; see
[`vim/README.md`](./vim/README.md) for details.

[Apache 2.0]: https://spdx.org/licenses/Apache-2.0.html
[OPUBL 1.0]: https://spdx.org/licenses/OPUBL-1.0.html
