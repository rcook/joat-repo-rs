# joat-repo-rs

[![CI](https://github.com/rcook/joat-repo-rs/actions/workflows/ci.yaml/badge.svg)][ci-workflow]
[![Publish](https://github.com/rcook/joat-repo-rs/actions/workflows/publish.yaml/badge.svg)][publish-workflow]
[![crates.io](https://img.shields.io/crates/v/joat-repo.svg)][crates-io]

Metadirectory repositories

## Use package

Get it from [crates.io][crates-io]:

```bash
cargo add joat-repo
```

## Build locally

```bash
git clone git@github.com:rcook/joat-repo-rs.git
cd joat-repo-rs
cargo build
cargo test
```

## Run example binary

```bash
git clone git@github.com:rcook/joat-repo-rs.git
cd joat-repo-rs
cargo run --features example-bin
```

[ci-workflow]: https://github.com/rcook/joat-repo-rs/actions/workflows/ci.yaml
[crates-io]: https://crates.io/crates/joat-repo
[publish-workflow]: https://github.com/rcook/joat-repo-rs/actions/workflows/publish.yaml
