mod builder;
mod escape;
mod formatter;
mod links;
mod list;
mod table;
mod utils;

pub use builder::FormatterBuilder;
pub use formatter::MarkdownFormatter;

/// Reformat a markdown snippet with all the default settings.
pub fn rewrite_markdown(input: &str) -> Result<String, std::io::Error> {
    rewrite_markdown_with_builder(input, FormatterBuilder::default())
}

/// Reformat a markdown snippet with user specified settings
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
