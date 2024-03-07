#![feature(rustc_private)]

// N.B. these crates are loaded from the sysroot, so they need extern crate.
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_builtin_macros;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_expand;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;
extern crate thin_vec;

// Necessary to pull in object code as the rest of the rustc crates are shipped only as rmeta
// files.
#[allow(unused_extern_crates)]
extern crate rustc_driver;

mod builder;
mod escape;
mod formatter;
mod links;
mod list;
pub mod rust_crate;
mod table;
mod utils;

pub use builder::FormatterBuilder;
pub use formatter::MarkdownFormatter;

/// Reformat a markdown snippet with all the default settings.
///
/// ```rust
/// # use markdown_fmt::rewrite_markdown;
/// let markdown = r##"  #   Learn Rust Checklist!
/// 1. Read [The Book]
///  2.  Watch tutorials
///   3.   Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let formatted_markdown = r##"# Learn Rust Checklist!
/// 1. Read [The Book]
/// 2. Watch tutorials
/// 3. Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/"##;
///
/// let output = rewrite_markdown(markdown).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown(input: &str) -> Result<String, std::io::Error> {
    rewrite_markdown_with_builder(input, FormatterBuilder::default())
}

/// Reformat a markdown snippet with user specified settings
///
/// ```rust
/// # use markdown_fmt::{rewrite_markdown_with_builder, FormatterBuilder};
/// let markdown = r##"  #   Learn Rust Checklist!
/// 1. Read [The Book]
///  2.  Watch tutorials
///   3.   Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let formatted_markdown = r##"# Learn Rust Checklist!
/// 1. Read [The Book]
/// 2. Watch tutorials
/// 3. Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/"##;
///
/// let builder = FormatterBuilder::default();
/// let output = rewrite_markdown_with_builder(markdown, builder).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown_with_builder(
    input: &str,
    builder: FormatterBuilder,
) -> Result<String, std::io::Error> {
    let formatter = builder.build();
    formatter.format(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reformat() {
        let input = r##"#  Hello World!
1.  Hey [ there! ]
2.  what's going on?

<p> and a little bit of HTML </p>

```rust
fn main() {}
```
[
    there!
    ]: htts://example.com "Yoooo"
"##;
        let expected = r##"# Hello World!
1. Hey [ there! ]
2. what's going on?

<p> and a little bit of HTML </p>

```rust
fn main() {}
```
[ there! ]: htts://example.com "Yoooo""##;
        let rewrite = rewrite_markdown(input).unwrap();
        assert_eq!(rewrite, expected)
    }
}
