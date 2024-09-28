# yamd

[![codecov](https://codecov.io/gh/Lurk/yamd/branch/main/graph/badge.svg?token=F8KRUYI1AA)](https://codecov.io/gh/Lurk/yamd)
[![crates.io](https://img.shields.io/crates/v/yamd.svg)](https://crates.io/crates/yamd)
[![Released API docs](https://docs.rs/yamd/badge.svg)](https://docs.rs/yamd)

<!-- cargo-rdme start -->

YAMD - Yet Another Markdown Document (flavour)

Simplified version of [CommonMark](https://spec.commonmark.org/).

For formatting check YAMD struct documentation.

## Reasoning

Simplified set of rules allows to have simpler, more efficient, parser and renderer.
YAMD does not provide render functionality, instead it is a [serde]
serializable structure that allows you to write any renderer for that structure. All HTML
equivalents in this doc are provided as an example to what it can be rendered.

## Difference from CommonMark

### Escaping

Escaping done on a [lexer] level. Every symbol following the `\` symbol will be treated as a
literal.

Example:

| YAMD      | HTML equivalent |
|-----------|-----------------|
| `\**foo**`|`<p>**foo**</p>` |

### Escape character

To get `\` - `\\` must be used.

Example:

| YAMD          | HTML equivalent       |
|---------------|-----------------------|
| `\\**foo**`   |`<p>\<b>foo</b></p>`   |


### Precedence

[CommonMark](https://spec.commonmark.org/0.31.2/#precedence) defines container blocks and leaf
blocks. And that container block indicator has higher precedence. YAMD does not discriminate by
block type, every node (block) is the same. In practice, there are no additional rules to encode
and remember.

Example:

| YAMD                  | HTML equivalent                               |
|-----------------------|-----------------------------------------------|
| ``- `one\n- two` ``   | `<ol><li><code>one\n- two</code></li></ol>`   |


If you want to have two ListItem's use escaping:

| YAMD                      | HTML equivalent                           |
|---------------------------|-------------------------------------------|
| ``- \`one\n- two\` ``     | ``<ol><li>`one</li><li>two`</li><ol>``    |

The reasoning is that those kind issues can be caught with tooling like linters/lsp. That tooling
does not exist yet.

### Nodes

List of supported [nodes](https://docs.rs/yamd/latest/yamd/nodes/) and their formatting slightly defers from CommonSpec.

<!-- cargo-rdme end -->
