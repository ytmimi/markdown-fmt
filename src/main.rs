#![feature(rustc_private)]

extern crate rustc_span;
use clap::Parser;
use markdown_fmt::rewrite_markdown;
use markdown_fmt::rust_crate::parse_crate::rewrite_doc_comments_in_crate;
use rustc_span::edition::Edition;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file. Markdown (.md) or Rust (.rs)
    input: PathBuf,
    /// Sets the edition value to pass when calling the rustc parser
    #[arg(short, long, default_value="2021", value_parser = ["2015", "2018", "2021", "2024"])]
    edition: String,
    /// Whether to emit output to stdout. Otherwise the content of the
    /// original file will be overwritten
    #[arg(long, default_value_t = false)]
    stdout: bool,
}

fn output_result(input: &Path, result: &str, stdout: bool) -> Result<(), anyhow::Error> {
    if stdout {
        print!("{result}");
        Ok(())
    } else {
        Ok(std::fs::write(input, result)?)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let edition = Edition::from_str(&cli.edition).expect("valid edition is passed");

    match cli.input.extension().and_then(OsStr::to_str) {
        Some("rs") => rustc_span::create_session_if_not_set_then(edition, |_| {
            if let Some(result) = rewrite_doc_comments_in_crate(&cli.input)? {
                return output_result(&cli.input, &result, cli.stdout);
            }
            Err(anyhow::anyhow!("Faile to rewrite crate doc comments"))
        }),
        Some("md") => {
            let input = fs::read_to_string(&cli.input)?;
            let result = rewrite_markdown(&input)?;
            output_result(&cli.input, &result, cli.stdout)
        }
        _ => Err(anyhow::anyhow!(
            "{} is not a markdown (.md) or rust (.rs) file.",
            cli.input.display()
        )),
    }
}
