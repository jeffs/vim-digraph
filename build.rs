//! Generates `digraph.rs` from `digraph.txt`.

#![deny(clippy::pedantic)]

use std::{env, path::Path};

#[path = "src/generate.rs"]
pub mod generate;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=vim/digraph.txt");

    let out_dir = env::var("OUT_DIR").expect("cargo should set OUT_DIR");
    generate::digraphs(
        &Path::new("vim").join("digraph.txt"),
        &Path::new(&out_dir).join("digraph-table.rs"),
    )
    .expect("transforming digraph table to Rust");
}
