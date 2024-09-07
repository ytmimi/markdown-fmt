#![allow(missing_docs)]

use clap::Parser;
use markdown_fmt::{rewrite_markdown_with_builder, FormatBuilder};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input Markdown (.md) file.
    input: PathBuf,
    /// Whether to emit output to stdout. Otherwise the content of the
    /// original file will be overwritten
    #[arg(long, default_value_t = false)]
    stdout: bool,
    /// The max width to use when reformatting paragraphs.
    #[arg(short, long)]
    max_width: Option<usize>,
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
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MARKDOWN_FMT_LOG"))
        .init();

    let cli = Cli::parse();

    match cli.input.extension().and_then(OsStr::to_str) {
        Some("md") => {
            let input = fs::read_to_string(&cli.input)?;
            let mut builder = FormatBuilder::default();
            builder.max_width(cli.max_width);
            let result = rewrite_markdown_with_builder(&input, builder)?;
            output_result(&cli.input, &result, cli.stdout)
        }
        _ => Err(anyhow::anyhow!(
            "{} is not a markdown (.md) or rust (.rs) file.",
            cli.input.display()
        )),
    }
}
