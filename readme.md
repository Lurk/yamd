# yamd

[![codecov](https://codecov.io/gh/Lurk/yamd/branch/main/graph/badge.svg?token=F8KRUYI1AA)](https://codecov.io/gh/Lurk/yamd)
[![crates.io](https://img.shields.io/crates/v/yamd.svg)](https://crates.io/crates/yamd)
[![Released API docs](https://docs.rs/yamd/badge.svg)](https://docs.rs/yamd)

<!-- cargo-rdme start -->

YAMD - Yet Another Markdown Document (flavour)

Simplified version of [CommonMark](https://spec.commonmark.org/).

For formatting check [`YAMD`](nodes::Yamd) struct documentation.

## Quick start

```rust
use yamd::deserialize;

let input = "# Hello\n\nA paragraph with **bold** text.";
let yamd = deserialize(input);

// Access the AST
assert_eq!(yamd.body.len(), 2);

// Round-trip back to markdown
assert_eq!(yamd.to_string(), input);
```

## Two APIs

- [`deserialize`] returns a nested [`Yamd`](nodes::Yamd) document — a tree of typed nodes,
  suitable for walking, pattern-matching, or round-tripping back to markdown via
  [`Display`](std::fmt::Display). The AST makes invalid nestings unrepresentable, and
  `deserialize` is fuzz-tested for panic-freedom and property-tested for round-trip fidelity.
- [`parse`] returns a flat `Vec<`[`Op`](op::Op)`>` of Start/End/Value events, where
  [`Content`](op::Content) borrows from the source when possible. Reach for it when you want
  streaming rendering or zero-copy text processing without materializing the full tree.
  [`to_yamd`] promotes an event stream to the tree form. Fuzz-tested for panic-freedom
  (transitively, via `deserialize`); the AST's type-level invariants and round-trip property
  do not apply at this layer.

## Reasoning

YAMD exchanges CommonMark's context-dependent rules for a uniform set: every node is treated
the same, and escaping is resolved at the lexer. The goal is a parser that's easier to reason
about locally, with fewer special cases to remember.

Rendering is out of scope; [`Yamd`](nodes::Yamd) is an AST you walk and render however you
like. With the `serde` feature enabled, the AST is also serde-serializable.

## Difference from CommonMark

YAMD reuses most of CommonMark's syntax but diverges in a few places.

### Escaping

Escaping is handled at the [lexer] level: any character following `\` is treated as a
[literal](lexer::TokenKind::Literal).

Example:

| YAMD      | HTML equivalent |
|-----------|-----------------|
| `\**foo**`|`<p>**foo**</p>` |

### Precedence

[CommonMark](https://spec.commonmark.org/0.31.2/#precedence) distinguishes container blocks from
leaf blocks and gives container-block markers higher precedence. YAMD does not distinguish block
types — every node is treated the same, so there are no precedence rules to remember.

Example:

| YAMD                  | HTML equivalent                               |
|-----------------------|-----------------------------------------------|
| ``- `one\n- two` ``   | `<ol><li><code>one\n- two</code></li></ol>`   |


To get two separate [ListItem](nodes::ListItem)s, escape the backticks:

| YAMD                      | HTML equivalent                           |
|---------------------------|-------------------------------------------|
| ``- \`one\n- two\` ``     | ``<ol><li>`one</li><li>two`</li><ol>``    |

The reasoning: issues like this should be caught by tooling such as linters or language servers
— that tooling doesn't exist yet.

### Nodes

See [nodes] for the full list of supported nodes and their formatting. Start with [YAMD](nodes::Yamd).

## MSRV

YAMD minimal supported Rust version is 1.87.

<!-- cargo-rdme end -->
