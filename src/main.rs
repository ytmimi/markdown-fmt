use clap::Parser;
use markdown_fmt::rewrite_markdown;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input Markdown (.md) file.
    input: PathBuf,
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

    match cli.input.extension().and_then(OsStr::to_str) {
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
