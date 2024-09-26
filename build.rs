#![allow(missing_docs)]

fn main() {
    generate_tests_markdown_tests().unwrap()
}

#[cfg(not(feature = "gen-tests"))]
fn generate_tests_markdown_tests() -> std::io::Result<()> {
    Ok(())
}

#[cfg(feature = "gen-tests")]
const PULLDOWN_CMARK_PREFIX: &str = "pulldown_cmark_";

#[cfg(feature = "gen-tests")]
const COMMONMARK_SPEC_URL: &str = "https://spec.commonmark.org/0.30/";

#[cfg(feature = "gen-tests")]
const GITHUB_FLAVORED_MARKDOWN_SPEC_URL: &str = "https://github.github.com/gfm/";

#[cfg(feature = "gen-tests")]
const PULLDOWN_CMARK_SPEC_URL: &str =
    "https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs";

/// How to link to the source example
#[cfg(feature = "gen-tests")]
#[derive(Debug, Clone, Copy)]
enum UrlKind {
    /// Link to an example from the commonmark spec as
    /// https://spec.commonmark.org/0.30/
    CommonMarkSpec,
    /// Link to an example from the GitHub flavored commonmark spec as
    /// https://github.github.com/gfm/
    GitHubFlavoredMarkdownSpec,
    /// Link to line numbers within the pulldown-cmark source code
    PulldownCmarkRepoSpec { filename: &'static str },
}

#[cfg(feature = "gen-tests")]
fn generate_tests_markdown_tests() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::PathBuf;

    let test_folder = "./tests/";

    let spec_files = [
        (
            "",
            "./tests/spec/CommonMark/commonmark_v0_30_spec.json",
            UrlKind::CommonMarkSpec,
        ),
        (
            "gfm_",
            "./tests/spec/GitHub/gfm_spec_v0_29_0_gfm_13.json",
            UrlKind::GitHubFlavoredMarkdownSpec,
        ),
        (
            PULLDOWN_CMARK_PREFIX,
            "./tests/spec/pulldown_cmark/footnotes_v0_10_3.json",
            UrlKind::PulldownCmarkRepoSpec {
                filename: "footnotes.txt",
            },
        ),
        (
            PULLDOWN_CMARK_PREFIX,
            "./tests/spec/pulldown_cmark/metadata_blocks_v0_10_3.json",
            UrlKind::PulldownCmarkRepoSpec {
                filename: "metadata_blocks.txt",
            },
        ),
        (
            PULLDOWN_CMARK_PREFIX,
            "./tests/spec/pulldown_cmark/heading_attrs_v0_10_3.json",
            UrlKind::PulldownCmarkRepoSpec {
                filename: "heading_attrs.txt",
            },
        ),
    ];

    for (prefix, spec, url) in spec_files {
        // FIXME(ytmimi) switch to `cargo::` build script syntax when the Minimum Supported Rust
        // Version (MSRV) is 1.77.0 or higher.
        println!("cargo:rerun-if-changed={}", spec);
        let spec_file = spec.split('/').last().unwrap();
        let mut output_file = PathBuf::from(format!("{test_folder}{spec_file}"));
        output_file.set_extension("rs");
        println!("output_file: {}", output_file.display());

        let test_cases: Vec<TestCase<'_>> = serde_json::from_reader(File::open(&spec)?)?;
        let mut output = BufWriter::new(File::create(&output_file)?);

        write_test_cases(&mut output, prefix, test_cases, url)
            .expect("generated test case successfully");
    }

    Ok(())
}

#[cfg(feature = "gen-tests")]
#[derive(Debug, serde::Deserialize)]
struct TestCase<'a> {
    #[serde(rename(deserialize = "markdown"))]
    input: std::borrow::Cow<'a, str>,
    #[serde(rename(deserialize = "formattedMarkdown"))]
    output: Option<std::borrow::Cow<'a, str>>,
    #[serde(rename(deserialize = "example"))]
    id: usize,
    section: std::borrow::Cow<'a, str>,
    #[serde(default)]
    skip: bool,
    #[serde(default = "default_test", rename(deserialize = "testMacro"))]
    test_macro: std::borrow::Cow<'a, str>,
    comment: Option<std::borrow::Cow<'a, str>>,
    start_line: usize,
    end_line: usize,
}

#[cfg(feature = "gen-tests")]
fn default_test() -> std::borrow::Cow<'static, str> {
    // Name of the test macro to use
    "test_identical_markdown_events".into()
}

#[cfg(feature = "gen-tests")]
fn write_test_cases<W>(
    writer: &mut W,
    prefix: &str,
    test_cases: Vec<TestCase<'_>>,
    url: UrlKind,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(writer, "// @generated")?;
    writeln!(writer, "// generated running `cargo build -F gen-tests`")?;
    writeln!(writer, "// test macros are defined in tests/common/mod.rs")?;
    // #![allow(missing_docs)] to work around missing_doc issue on nightly
    writeln!(writer, "#![allow(missing_docs)]")?;
    writeln!(writer, "mod common;")?;

    for test_case in test_cases.into_iter() {
        write_test_case(writer, prefix, test_case, url)?;
    }
    Ok(())
}

#[cfg(feature = "gen-tests")]
fn write_test_case<W: std::io::Write>(
    writer: &mut W,
    prefix: &str,
    test_case: TestCase<'_>,
    url: UrlKind,
) -> std::io::Result<()> {
    let url = match url {
        UrlKind::CommonMarkSpec => {
            format!("{COMMONMARK_SPEC_URL}#example-{}", test_case.id)
        }
        UrlKind::GitHubFlavoredMarkdownSpec => {
            format!(
                "{GITHUB_FLAVORED_MARKDOWN_SPEC_URL}#example-{}",
                test_case.id
            )
        }
        UrlKind::PulldownCmarkRepoSpec { filename } => {
            format!(
                "{PULLDOWN_CMARK_SPEC_URL}/{filename}#L{}-L{}",
                test_case.start_line, test_case.end_line
            )
        }
    };

    let replace_tab_chars = test_case.input.replace('→', "\t");
    let input = replace_tab_chars.trim_end_matches('\n');

    if let Some(comment) = test_case.comment {
        write!(writer, "\n// {comment}")?;
    }

    if test_case.skip {
        write!(writer, "\n#[ignore]")?;
    }

    write!(
        writer,
        r##"
#[test]
fn {}markdown_{}_{}() {{
    // {}
    {}!("##,
        prefix,
        test_case
            .section
            .to_lowercase()
            .replace(char::is_whitespace, "_")
            .replace("(", "")
            .replace(")", ""),
        test_case.id,
        url,
        test_case.test_macro,
    )?;

    let has_trailing_whitespace = input.lines().any(|l| l.ends_with(char::is_whitespace));
    if has_trailing_whitespace {
        write!(writer, "{:?}", input)?;
    } else {
        write!(writer, "r##\"{}\"##", input)?;
    }
    if let Some(expected_output) = test_case.output {
        let has_trailing_whitespace = expected_output
            .lines()
            .any(|l| l.ends_with(char::is_whitespace));
        if has_trailing_whitespace {
            write!(writer, ",{:?}", expected_output)?;
        } else {
            write!(writer, ",r##\"{}\"##", expected_output)?;
        }
    }
    write!(writer, ");")?;
    write!(writer, "\n}}\n")?;
    Ok(())
}
