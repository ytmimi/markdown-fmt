use crate::{rewrite_markdown, rewrite_markdown_with_builder, FormatterBuilder};
use rust_search::SearchBuilder;
use std::path::{Path, PathBuf};

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
1. Hey [there!]
2. what's going on?

<p> and a little bit of HTML </p>

```rust
fn main() {}
```
[there!]: htts://example.com "Yoooo"
"##;
    let rewrite = rewrite_markdown(input).unwrap();
    assert_eq!(rewrite, expected)
}

fn get_test_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = PathBuf> {
    SearchBuilder::default()
        .ext(".md")
        .location(path)
        .build()
        .map(|f| PathBuf::from(f))
}

#[test]
fn check_markdown_formatting() {
    let mut errors = 0;

    for file in get_test_files("tests/source") {
        let input = std::fs::read_to_string(&file).unwrap();
        let builder = FormatterBuilder::default();
        let formatted_input = rewrite_markdown_with_builder(&input, builder).unwrap();
        let target_file = file
            .strip_prefix("tests/source")
            .map(|p| PathBuf::from("tests/target").join(p))
            .unwrap();
        let expected_output = std::fs::read_to_string(target_file).unwrap();

        if formatted_input != expected_output {
            errors += 1;
        }
    }

    assert_eq!(errors, 0, "there should be no formatting error");
}

#[test]
fn idempotence_test() {
    let mut errors = 0;

    for file in get_test_files("tests/target") {
        let input = std::fs::read_to_string(&file).unwrap();
        let builder = FormatterBuilder::default();
        let formatted_input = rewrite_markdown_with_builder(&input, builder).unwrap();

        if formatted_input != input {
            errors += 1;
        }
    }

    assert_eq!(errors, 0, "formatting should not change in target files");
}
