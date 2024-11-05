use crate::config::{Config, ConfigSetError};
use crate::{FormatBuilder, rewrite_markdown, rewrite_markdown_with_builder};
use rust_search::SearchBuilder;
use std::collections::HashMap;
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
    fn from_leading_config_comments(input: &str) -> Result<Self, ConfigSetError> {
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
            config.set(config_option, value.trim())?;
        }

        let mut builder = FormatBuilder::default();
        builder.config(config);
        Ok(builder)
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

    // Additional features required to run this test
    let mut missing_features: HashMap<&'static str, Vec<PathBuf>> = HashMap::new();

    // Any unexpected configuration. Probably a typo that should be error on.
    let mut unknown_configs: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in get_test_files("tests/source", "md") {
        let input = std::fs::read_to_string(&file).unwrap();
        let builder = match FormatBuilder::from_leading_config_comments(&input) {
            Ok(b) => b,
            Err(e) => {
                match e {
                    ConfigSetError::MissingFeature(name) => {
                        missing_features.entry(name.into()).or_default().push(file);
                    }
                    ConfigSetError::UnknownConfig(name) => {
                        unknown_configs
                            .entry(name.to_string())
                            .or_default()
                            .push(file);
                    }
                }
                continue;
            }
        };
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

    for (feature, files) in missing_features {
        for file in files {
            eprintln!(
                "\n\twarning can't check markdown formatting for {}. Must enable `-F {}`",
                file.display(),
                feature
            );
        }
    }

    assert_eq!(errors, 0, "there should be no formatting error");

    let any_unknown_features = !unknown_configs.is_empty();

    for (feature, files) in unknown_configs {
        for file in files {
            eprintln!(
                "\n\terror check markdown formatting for {}. Unknown feature `{}`",
                file.display(),
                feature
            );
        }
    }

    assert!(!any_unknown_features, "there should be no unknown features");
}

#[test]
fn idempotence_test() {
    let mut errors = 0;

    // Additional features required to run this test
    let mut missing_features: HashMap<&'static str, Vec<PathBuf>> = HashMap::new();

    // Any unexpected configuration. Probably a typo that should be error on.
    let mut unknown_configs: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in get_test_files("tests/target", "md") {
        let input = std::fs::read_to_string(&file).unwrap();
        let builder = match FormatBuilder::from_leading_config_comments(&input) {
            Ok(b) => b,
            Err(e) => {
                match e {
                    ConfigSetError::MissingFeature(name) => {
                        missing_features.entry(name.into()).or_default().push(file);
                    }
                    ConfigSetError::UnknownConfig(name) => {
                        unknown_configs
                            .entry(name.to_string())
                            .or_default()
                            .push(file);
                    }
                }
                continue;
            }
        };
        let formatted_input = rewrite_markdown_with_builder(&input, builder).unwrap();

        if formatted_input != input {
            errors += 1;
            eprintln!("error formatting {}", file.display());
        }
    }

    for (feature, files) in missing_features {
        for file in files {
            eprintln!(
                "\n\twarning can't run idempotence test for {}. Must enable `-F {}`",
                file.display(),
                feature
            );
        }
    }

    assert_eq!(errors, 0, "formatting should not change in target files");

    let any_unknown_features = !unknown_configs.is_empty();

    for (feature, files) in unknown_configs {
        for file in files {
            eprintln!(
                "\n\terror can't run idempotence test for {}. Unknown feature `{}`",
                file.display(),
                feature
            );
        }
    }

    assert!(!any_unknown_features, "there should be no unknown features");
}
