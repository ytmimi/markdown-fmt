#![allow(missing_docs)]

use clap::Parser;
use markdown_fmt::{FormatBuilder, rewrite_markdown_with_builder};
use std::fs;
use std::path::{Path, PathBuf};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

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
    /// Should text reflow when max width is also configured.
    #[arg(short, long)]
    reflow_text: bool,
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

    let input = fs::read_to_string(&cli.input)?;
    let mut builder = FormatBuilder::default();
    builder
        .max_width(cli.max_width)
        .reflow_text(cli.reflow_text);
    let result = rewrite_markdown_with_builder(&input, builder)?;
    output_result(&cli.input, &result, cli.stdout)
}
