# dir-aspect

[![crates.io](https://img.shields.io/crates/v/dir-aspect)](https://crates.io/crates/dir-aspect)
[![docs.rs](https://img.shields.io/docsrs/dir-aspect)](https://docs.rs/dir-aspect)
[![CI](https://github.com/sksizer/rust-dir-aspect/actions/workflows/ci.yml/badge.svg)](https://github.com/sksizer/rust-dir-aspect/actions/workflows/ci.yml)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Detect what kind of application folder a directory is. Checks for marker
directories and files (`.git`, `.obsidian/`) and reports the "aspects" a
directory has, so desktop tooling can enable behavior specific to that kind of
project.

Use it when a tool is pointed at a folder and must know whether it is a Git
checkout or an Obsidian vault. It checks markers only: it does not read Git
state or parse a vault.

## Install

```sh
cargo add dir-aspect
```

## Usage

```rust
use dir_aspect::{detect_aspects, Aspect};

let aspects = detect_aspects(std::path::Path::new("/path/to/dir"));
if aspects.contains(&Aspect::Git) {
    println!("git repository");
}
if aspects.contains(&Aspect::Obsidian) {
    println!("obsidian vault");
}
```

Every directory carries `Aspect::Generic`. `Obsidian` is added when `.obsidian/`
is a directory, and `Git` when `.git` exists, as a directory or as the file a
linked worktree or submodule has.

For an Obsidian vault, the vault id can be read from Obsidian's own config
(`obsidian.json`):

```rust
use dir_aspect::detect_obsidian_vault_id;

if let Some(id) = detect_obsidian_vault_id(std::path::Path::new("/path/to/vault")) {
    // e.g. deep-link into Obsidian: obsidian://open?vault-id=...
    println!("vault id: {id}");
}
```

A runnable version is in [`examples/detect.rs`](examples/detect.rs):
`cargo run --example detect -- <dir>`.

## Features

| Feature | Effect |
|---|---|
| `specta` | Derives [`specta::Type`](https://docs.rs/specta) on `Aspect` for typed TypeScript bindings. Off by default. |

`Aspect` serializes with serde as lowercase strings (`"git"`, `"obsidian"`,
`"generic"`).

## Development

```sh
just full-check
just test
```

Releases are made by release-plz from conventional commits; see
[RELEASE.md](RELEASE.md).

## License

[MIT](LICENSE-MIT)
