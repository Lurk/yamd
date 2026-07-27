# yamd

[![codecov](https://codecov.io/gh/Lurk/yamd/branch/main/graph/badge.svg?token=F8KRUYI1AA)](https://codecov.io/gh/Lurk/yamd)
[![crates.io](https://img.shields.io/crates/v/yamd.svg)](https://crates.io/crates/yamd)
[![Released API docs](https://docs.rs/yamd/badge.svg)](https://docs.rs/yamd)

<!-- cargo-rdme start -->

YAMD - Yet Another Markdown Document (flavour)

Simplified version of [CommonMark](https://spec.commonmark.org/).

For formatting check [`YAMD`](https://docs.rs/yamd/latest/yamd/nodes/yamd/struct.Yamd.html) struct documentation.

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

- [`deserialize`](https://docs.rs/yamd/latest/yamd/fn.deserialize.html) returns a nested [`Yamd`](https://docs.rs/yamd/latest/yamd/nodes/yamd/struct.Yamd.html) document — a tree of typed nodes,
  suitable for walking, pattern-matching, or round-tripping back to markdown via
  [`Display`](https://doc.rust-lang.org/stable/core/fmt/trait.Display.html). The AST makes invalid nestings unrepresentable, and
  `deserialize` is fuzz-tested for panic-freedom and property-tested for round-trip fidelity.
- [`parse`](https://docs.rs/yamd/latest/yamd/op/fn.parse.html) returns a flat `Vec<`[`Op`](https://docs.rs/yamd/latest/yamd/op/struct.Op.html)`>` of Start/End/Value events.
  [`to_yamd`](https://docs.rs/yamd/latest/yamd/op/to_yamd/fn.to_yamd.html) promotes an event stream to the tree form. Fuzz-tested for panic-freedom
  (transitively, via `deserialize`); the AST's type-level invariants and round-trip property
  do not apply at this layer.

Rendering is out of scope; [`Yamd`](https://docs.rs/yamd/latest/yamd/nodes/yamd/struct.Yamd.html) is an AST you walk and render however you
like. With the `serde` feature enabled, the AST is also serde-serializable.

## Difference from CommonMark

YAMD reuses most of CommonMark's syntax but diverges where CommonMark's context-dependent
rules would force special cases: every node is treated the same (no container/leaf
distinction), and escaping is context-independent.

### Escaping

Escaping is handled at the [`lexer`](https://docs.rs/yamd/latest/yamd/lexer/) level: any character following `\` is treated as a
[literal](https://docs.rs/yamd/latest/yamd/lexer/token/enum.TokenKind.html#variant.Literal).

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


To get two separate [`ListItem`](https://docs.rs/yamd/latest/yamd/nodes/list_item/struct.ListItem.html)s, escape the backticks:

| YAMD                      | HTML equivalent                           |
|---------------------------|-------------------------------------------|
| ``- \`one\n- two\` ``     | ``<ol><li>`one</li><li>two`</li><ol>``    |

The reasoning: issues like this should be caught by tooling such as linters or language servers
— that tooling doesn't exist yet.

### Nodes

See [`nodes`](https://docs.rs/yamd/latest/yamd/nodes/) for the full list of supported nodes and their formatting. Start with [YAMD](https://docs.rs/yamd/latest/yamd/nodes/yamd/struct.Yamd.html).

## MSRV

YAMD minimal supported Rust version is 1.87.

<!-- cargo-rdme end -->
