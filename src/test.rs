use crate::config::Config;
use crate::{FormatBuilder, rewrite_markdown, rewrite_markdown_with_builder};
use rust_search::SearchBuilder;
use std::path::{Path, PathBuf};

impl FormatBuilder {
    /// Configure the FormatBuilder using leading comments in test files.
    ///
    /// For example:
    /// ```markdown
    /// <!-- :max_width:50 -->
    ///
    /// Paragraphs will now wrap at a column width of 50.
    /// ```
    pub fn from_leading_config_comments(input: &str) -> Self {
        let mut config = Config::default();

        let opener = "<!-- :";
        let closer = "-->";
        for l in input
            .lines()
            .take_while(|l| l.starts_with(opener) && l.ends_with(closer))
        {
            let Some((config_option, value)) = l[opener.len()..l.len() - closer.len()]
                .trim()
                .split_once(':')
            else {
                continue;
            };
            config.set(config_option, value.trim());
        }

        let mut builder = FormatBuilder::default();
        builder.config(config);
        builder
    }
}

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

pub(crate) fn get_test_files<P: AsRef<Path>>(
    path: P,
    extension: &str,
) -> impl Iterator<Item = PathBuf> {
    SearchBuilder::default()
        .ext(extension)
        .location(path)
        .build()
        .map(PathBuf::from)
}

#[test]
fn check_markdown_formatting() {
    let mut errors = 0;

    for file in get_test_files("tests/source", "md") {
        let input = std::fs::read_to_string(&file).unwrap();
        let builder = FormatBuilder::from_leading_config_comments(&input);
        let rewrite = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            rewrite_markdown_with_builder(&input, builder).unwrap()
        }));

        let target_file = file
            .strip_prefix("tests/source")
            .map(|p| PathBuf::from("tests/target").join(p))
            .unwrap();
        let expected_output = std::fs::read_to_string(target_file).unwrap();

        let Ok(formatted_output) = rewrite else {
            panic!("Paniced when formatting {}", file.display())
        };

        if formatted_output != expected_output {
            errors += 1;
            eprintln!("error formatting {}", file.display());
        }
    }

    assert_eq!(errors, 0, "there should be no formatting error");
}

#[test]
fn idempotence_test() {
    let mut errors = 0;

    for file in get_test_files("tests/target", "md") {
        let input = std::fs::read_to_string(&file).unwrap();
        let builder = FormatBuilder::from_leading_config_comments(&input);
        let formatted_input = rewrite_markdown_with_builder(&input, builder).unwrap();

        if formatted_input != input {
            errors += 1;
            eprintln!("error formatting {}", file.display());
        }
    }

    assert_eq!(errors, 0, "formatting should not change in target files");
}

#[cfg(test)]
mod tester {
    use crate::rewrite_markdown;

    #[test]
    fn test_edge_cases() {
        let result = rewrite_markdown(">\rsome text").unwrap();
        assert_eq!(result, ">\nsome text");

        let result = rewrite_markdown("##").unwrap();
        assert_eq!(result, "#");

        let input = "<?*?'\n  \n  ";
        let result = rewrite_markdown(input).unwrap();
        assert_eq!(result, "<?*?'\n\n");

        let result = rewrite_markdown("[`\r``]").unwrap();
        assert_eq!(result, "[` ``]");
    }
}
