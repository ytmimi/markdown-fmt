# markdown-fmt

A library that applies a light touch when formatting markdown documents.
The project supports [CommonMark] and [GitHub Flavored Markdown]

[CommonMark]: https://spec.commonmark.org/
[GitHub Flavored Markdown]: https://github.github.com/gfm/


## How To Use

If you don't need to configure anything and just want to get started, you can use the the high-level
`rewrite_markdown` function.

```rust
use markdown_fmt::rewrite_markdown;

let markdown = r##"  # Getting Started
1. numbered lists
1.  are easy!

> > and so are block quotes.
"##;

let expected = r##"# Getting Started
1. numbered lists
1. are easy!

>> and so are block quotes.
"##;

let output = rewrite_markdown(markdown).unwrap();
assert_eq!(output, expected);
```

If you need more control over markdown formatting you can use the `FormatterBuilder` and the
`rewrite_markdown_with_builder` function.


```rust
use markdown_fmt::{FormatterBuilder, rewrite_markdown_with_builder};

let markdown = r##"# The standard Lorem Ipsum passage, used since the 1500s

"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
"##;

let expected = r##"# The standard Lorem Ipsum passage, used since the 1500s

"Lorem ipsum dolor sit amet, consectetur
adipiscing elit, sed do eiusmod tempor incididunt
ut labore et dolore magna aliqua. Ut enim ad minim
veniam, quis nostrud exercitation ullamco laboris
nisi ut aliquip ex ea commodo consequat. Duis aute
irure dolor in reprehenderit in voluptate velit
esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident,
sunt in culpa qui officia deserunt mollit anim id
est laborum."
"##;

let mut builder = FormatterBuilder::default();
builder.max_width(Some(50));

let output = rewrite_markdown_with_builder(markdown, builder).unwrap();
assert_eq!(output, expected);
```
