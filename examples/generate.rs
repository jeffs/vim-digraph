//! This script generates `digraph.rs` from `digraph.txt`.

use std::{env, path::Path, process::ExitCode};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> ExitCode {
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    let [table_rs] = &args[..] else {
        eprintln!("USAGE: generate [OUTPUT_PATH]");
        return ExitCode::FAILURE;
    };

    vim_digraph::generate::digraphs(
        &Path::new(MANIFEST_DIR).join("vim").join("digraph.txt"),
        Path::new(table_rs),
    )
    .expect("transforming digraph table to Rust");

    ExitCode::SUCCESS
}
