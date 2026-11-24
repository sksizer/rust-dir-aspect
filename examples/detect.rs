//! Prints the aspects of a directory, and its Obsidian vault id if it has one.
//!
//! Run with `cargo run --example detect -- <dir>`. With no argument it checks
//! the current directory.

use std::path::PathBuf;

use dir_aspect::{Aspect, detect_aspects, detect_obsidian_vault_id};

fn main() {
    let dir = std::env::args_os().nth(1).map_or_else(|| PathBuf::from("."), PathBuf::from);

    let aspects = detect_aspects(&dir);
    println!("{}: {aspects:?}", dir.display());

    if aspects.contains(&Aspect::Obsidian) {
        match detect_obsidian_vault_id(&dir) {
            Some(id) => println!("obsidian vault id: {id}"),
            None => println!("obsidian vault id: not registered with Obsidian"),
        }
    }
}
