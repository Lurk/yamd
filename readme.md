# yamd

[![codecov](https://codecov.io/gh/Lurk/yamd/branch/main/graph/badge.svg?token=F8KRUYI1AA)](https://codecov.io/gh/Lurk/yamd)
[![crates.io](https://img.shields.io/crates/v/yamd.svg)](https://crates.io/crates/yamd)
[![Released API docs](https://docs.rs/yamd/badge.svg)](https://docs.rs/yamd)

<!-- cargo-rdme start -->

YAMD - Yet Another Markdown Document (flavour)

Simplified version of [CommonMark](https://spec.commonmark.org/).

For formatting check [`YAMD`](nodes::Yamd) struct documentation.

## Reasoning

Simplified set of rules allows to have simpler, more efficient, parser and renderer.
[YAMD](nodes::Yamd) does not provide render functionality, instead it is a [serde]
serializable structure that allows you to write any renderer for that structure. All HTML
equivalents in this doc are provided as an example to what it can be rendered.

## Difference from CommonMark

While YAMD tries to utilize as much CommonMark syntax as possible, there are differences.

### Escaping

Escaping done on a [lexer] level. Every symbol following the `\` symbol will be treated as a
[literal](lexer::TokenKind::Literal).

Example:

| YAMD      | HTML equivalent |
|-----------|-----------------|
| `\**foo**`|`<p>**foo**</p>` |

### Precedence

[CommonMark](https://spec.commonmark.org/0.31.2/#precedence) defines container blocks and leaf
blocks. And that container block indicator has higher precedence. YAMD does not discriminate by
block type, every node (block) is the same. In practice, there are no additional rules to encode
and remember.

Example:

| YAMD                  | HTML equivalent                               |
|-----------------------|-----------------------------------------------|
| ``- `one\n- two` ``   | `<ol><li><code>one\n- two</code></li></ol>`   |


If you want to have two [ListItem](nodes::ListItem)'s use escaping:

| YAMD                      | HTML equivalent                           |
|---------------------------|-------------------------------------------|
| ``- \`one\n- two\` ``     | ``<ol><li>`one</li><li>two`</li><ol>``    |

The reasoning is that those kind issues can be caught with tooling like linters/lsp. That tooling
does not exist yet.

### Nodes

List of supported [nodes] and their formatting. The best starting point is [YAMD](nodes::Yamd).

## MSRV

YAMD minimal supported Rust version is 1.80.0 due to [Option::take_if] usage

<!-- cargo-rdme end -->
