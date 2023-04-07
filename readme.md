# Yet another markdown flavour (YAMD)
[![codecov](https://codecov.io/gh/Lurk/yamd/branch/main/graph/badge.svg?token=F8KRUYI1AA)](https://codecov.io/gh/Lurk/yamd)
[![crates.io](https://img.shields.io/crates/v/yamd.svg)](https://crates.io/crates/yamd)
[![Released API docs](https://docs.rs/yamd/badge.svg)](https://docs.rs/yamd)

## Status

It is ready to poke around. There is no significant API changes expected.

## Why?

Initial idea was to create human readable text format for my blog. Why not existing flavour? 
Existing flavours do not have elements like image gallery, dividers, highlight, etc.

## Features

Deserialize markdown to YAMD struct, Serialize YAMD struct to markdown.

## Example

```rust
use yamd::{deserialize, serialize};
let input = r#"# This is a new Yamd document

Check out [documentation](https://docs.rs/yamd/latest/yamd/) to get what elements **Yamd** format supports."#;
let yamd = deserialize(input).unwrap();
let output = serialize(&yamd);
```



