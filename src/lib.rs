//! Easily format Markdown. [markdown_fmt] supports [CommonMark] and [GitHub Flavored Markdown].
//!
//! [markdown_fmt]: index.html
//! [CommonMark]: https://spec.commonmark.org/
//! [GitHub Flavored Markdown]: https://github.github.com/gfm/
//!
//! # Getting Started
//!
//! ```rust
//! use markdown_fmt::rewrite_markdown;
//!
//! let markdown = r##" # Getting Started
//! 1. numbered lists
//! 1.  are easy!
//! "##;
//!
//! let formatted = r##"# Getting Started
//! 1. numbered lists
//! 1. are easy!
//! "##;
//!
//! let output = rewrite_markdown(markdown)?;
//! # assert_eq!(output, formatted);
//! # Ok::<(), std::fmt::Error>(())
//! ```
//!
//! # Using the [Builder](builder::FormatBuilder)
//!
//! The builder gives you more control to configure Markdown formatting.
//!
//! ````rust
//! use markdown_fmt::{rewrite_markdown, rewrite_markdown_with_builder, FormatBuilder};
//!
//! let builder = FormatBuilder::with_code_block_formatter(|_ctx, info_string, code_block| {
//!     match info_string.to_lowercase().as_str() {
//!         "markdown" => rewrite_markdown(&code_block).unwrap_or(code_block),
//!         _ => code_block
//!     }
//! });
//!
//! let markdown = r##" # Using the Builder
//! + markdown code block nested in a list
//!   ```markdown
//!   A nested markdown snippet!
//!
//!    * unordered lists
//!    are also pretty easy!
//!    - `-` or `+` can also be used as unordered list markers.
//!    ```
//! "##;
//!
//! let formatted = r##"# Using the Builder
//! + markdown code block nested in a list
//!   ```markdown
//!   A nested markdown snippet!
//!
//!   * unordered lists
//!     are also pretty easy!
//!   - `-` or `+` can also be used as unordered list markers.
//!   ```
//! "##;
//!
//! let output = rewrite_markdown_with_builder(markdown, builder)?;
//! # assert_eq!(output, formatted);
//! # Ok::<(), std::fmt::Error>(())
//! ````

mod adapters;
mod builder;
mod config;
mod definition_list;
mod escape;
mod footnote;
mod formatter;
mod header;
mod html;
mod links;
mod list;
mod paragraph;
mod table;
#[cfg(test)]
mod test;
mod utils;
mod writer;

pub use builder::{CodeBlockContext, FormatBuilder};
pub use formatter::MarkdownFormatter;

// Used for doctests in the README
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadMe;

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
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let output = rewrite_markdown(markdown).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown(input: &str) -> Result<String, std::fmt::Error> {
    rewrite_markdown_with_builder(input, FormatBuilder::default())
}

/// Reformat a markdown snippet with user specified settings
///
/// ```rust
/// # use markdown_fmt::{rewrite_markdown_with_builder, FormatBuilder};
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
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let builder = FormatBuilder::default();
/// let output = rewrite_markdown_with_builder(markdown, builder).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown_with_builder(
    input: &str,
    builder: FormatBuilder,
) -> Result<String, std::fmt::Error> {
    let formatter = builder.build();
    formatter.format(input)
}
