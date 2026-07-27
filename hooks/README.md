# Git hooks

Local checks that mirror CI (`.github/workflows/check.yml`):

- `pre-commit`: `cargo fmt --check`, `cargo clippy`, `cargo test`, `cargo rdme --check`
- `pre-push`: `cargo semver-checks`, `cargo hack check --feature-powerset`, MSRV check (version read from `Cargo.toml`'s `rust-version`)

## Setup

```sh
git config core.hooksPath hooks
```

## Required tooling

```sh
rustup component add rustfmt clippy
cargo install cargo-rdme --version 2.1.0
cargo rdme install-rust-toolchain-for-intralinks
cargo install cargo-semver-checks cargo-hack
rustup toolchain install "$(grep '^rust-version' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')"
```

