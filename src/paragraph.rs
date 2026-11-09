use crate::escape::needs_escape;
use crate::links::is_balanced;
use crate::table::count_table_cells;
use crate::utils::sequence_ends_on_escape;
use crate::writer::{MarkdownContext, WriteContext};

use pulldown_cmark::{Event, LinkType, Tag};
use regex::Regex;
use std::fmt::Write;
use std::sync::LazyLock;
use textwrap::Options as TextWrapOptions;

// FIXME(ytmimi)
// pulldown-cmark is more liberal about whitespace in GitHub Flavored Markdown Tables
// so the regex has a lot of optional whitespace. If that changes we could tighten up
// this regex.
// See <https://github.com/pulldown-cmark/pulldown-cmark/issues/1191>
static TABLE_DELIMITER_ROW_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let left_alignemnt = r"\:[-\s]*-?";
    let right_aligment = r"[-\s]*-?\s*\:";
    let center_aligment = r"\:[-\s]*-\s*\:";
    let no_aligment = r"[-\s]*";
    let regex = format!(
        r"^(?:{}|{}|{}|{})$",
        left_alignemnt, right_aligment, center_aligment, no_aligment,
    );
    Regex::new(&regex).expect("valid regex")
});

const MARKDOWN_HARD_BREAK: &str = "  \n";

/// A buffer where we write text
#[derive(Debug, PartialEq)]
pub(super) struct Paragraph {
    buffer: String,
    max_width: Option<usize>,
    should_reflow_text: bool,
}

impl WriteContext<'_> for Paragraph {
    fn write_context_str(&mut self, ctx: MarkdownContext, s: &str) -> std::fmt::Result {
        // We should only need to escape `Event::Text`, and multi-line `Event::InlineHtml` and
        // `Event::Code` since they might contain characters that look like other Markdown
        // constructs, but they're really just text.
        match ctx {
            MarkdownContext::Event(Event::Text(_) | Event::InlineHtml(_)) => {
                if s.is_empty() {
                    return self.write_str(s);
                }
            }
            MarkdownContext::Event(Event::Code(_)) => {
                if s.starts_with(char::is_whitespace) || s.is_empty() {
                    return self.write_str(s);
                }
            }
            MarkdownContext::Tag(Tag::Link { link_type, .. }) => {
                let closing_brace_needs_escape = || {
                    if !self.buffer.ends_with(']')
                        || matches!(link_type, LinkType::Autolink | LinkType::Email)
                    {
                        return false;
                    }

                    let Some(last_line) = self.buffer.lines().last() else {
                        return false;
                    };

                    for (opener_idx, _) in last_line.match_indices('[') {
                        if sequence_ends_on_escape(&last_line[..opener_idx]) {
                            continue;
                        }

                        let bracket_snippet = &last_line[opener_idx..];
                        if is_balanced(bracket_snippet, '[', ']') {
                            let trimmed_bracket_snippet = bracket_snippet
                                .trim_end_matches(['\n', ']'])
                                .trim_start_matches('[');

                            let only_whitespace_brace = !trimmed_bracket_snippet.is_empty()
                                && trimmed_bracket_snippet.chars().all(char::is_whitespace);

                            let starts_with_caret = trimmed_bracket_snippet.starts_with('^');

                            if only_whitespace_brace || starts_with_caret {
                                return true;
                            }
                        }
                    }
                    false
                };

                if closing_brace_needs_escape() {
                    // pop off the `]` and push it back escaped.
                    self.buffer.pop();
                    self.buffer.push_str("\\]");
                }

                return self.write_str(s);
            }
            _ => {
                return self.write_str(s);
            }
        }

        // FIXME(ytmimi) I'm adding alot of checks here. They mostly duplicate what's defined
        // in `needs_escape`, but only apply in certain scenarios. There's probably a much
        // better way to handle this.

        // Prevent the next pass of the parser from accidentaly interpreting a table
        // without a leading |
        if self.buffer.lines().last().is_some_and(|l| {
            let header_row_cells = count_table_cells(l);
            // For this to accidentally be treated as a table after formatting
            // the row above needs to look like it's divided into cells with `|`
            // and the current row (treated as the delimiter row) needs to have the
            // same number of cells.
            header_row_cells != 0 && header_row_cells == count_table_cells(s)
        }) && could_be_table(s)
        {
            self.buffer.push('\\');
        }

        let needs_escape = needs_escape(s).is_some();

        // Prevent the next pass from ignoring the hard break or misinterpreting `s`
        // as something other than text in a paragraph
        if self.buffer.ends_with(MARKDOWN_HARD_BREAK) && needs_escape {
            self.buffer.push('\\');
        }

        if self.buffer.ends_with('\n') && s.starts_with('#') && needs_escape {
            self.buffer.push('\\');
        }

        let all_chars_eq =
            |input: &str, marker: char| -> bool { input.chars().all(|c| c == marker) };

        // Prevent the next pass from interpreting this as a header
        if self.buffer.ends_with('\n') && (all_chars_eq(s, '-') || all_chars_eq(s, '=')) {
            self.buffer.push('\\');
        }

        // Prevent the next pass from interpreting this as a list
        if self.buffer.ends_with('\n') && matches!(s, "* " | "+ " | "- ") {
            self.buffer.push('\\');
        }

        let is_thematic_break = |input: &str, marker: char| -> bool {
            input
                .chars()
                .all(|c| matches!(c, ' ' | '\t') || c == marker)
                && input.chars().filter(|c| *c == marker).count() >= 3
        };

        if (self.buffer.ends_with('\n') || self.buffer.is_empty())
            && ['-', '_', '*'].iter().any(|c| is_thematic_break(s, *c))
        {
            self.buffer.push('\\');
        }

        self.write_str(s)
    }
}

impl Write for Paragraph {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.max_width.is_some() && self.should_reflow_text && s.trim().is_empty() {
            // If the user configured the max_width and the reflow text option
            // then push a space so we can reflow text
            self.buffer.push(' ');
        } else {
            self.buffer.push_str(s);
        }

        Ok(())
    }
}

impl Paragraph {
    pub(super) fn new(max_width: Option<usize>, should_reflow_text: bool, capacity: usize) -> Self {
        Self {
            max_width,
            buffer: String::with_capacity(capacity),
            should_reflow_text,
        }
    }

    /// Check if the internal buffer is empty
    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Consume Self and return the formatted buffer
    pub(super) fn into_buffer(mut self) -> String {
        let rewrite_buffer = std::mem::take(&mut self.buffer);

        let Some(max_width) = self.max_width else {
            // We didn't configure a max_width, so just return the buffer
            return rewrite_buffer;
        };

        let all_lines_with_max_width = rewrite_buffer.lines().all(|l| l.len() <= max_width);

        if all_lines_with_max_width {
            // Don't need to wrap any lines
            return rewrite_buffer;
        }

        let mut output_buffer = String::with_capacity(rewrite_buffer.capacity());

        let wrap_options = TextWrapOptions::new(max_width)
            .break_words(false)
            .word_separator(textwrap::WordSeparator::AsciiSpace)
            .wrap_algorithm(textwrap::WrapAlgorithm::FirstFit);

        let mut split_on_hard_breaks = rewrite_buffer.split(MARKDOWN_HARD_BREAK).peekable();

        while let Some(text) = split_on_hard_breaks.next() {
            let has_next = split_on_hard_breaks.peek().is_some();
            let wrapped_text = if self.should_reflow_text {
                let (text, _) = textwrap::unfill(text);
                textwrap::fill(&text, wrap_options.clone())
            } else {
                textwrap::fill(text, wrap_options.clone())
            };
            output_buffer.push_str(&wrapped_text);
            if has_next {
                output_buffer.push_str(MARKDOWN_HARD_BREAK);
            }
        }

        output_buffer
    }
}

/// Determine if this is the delimiter row of a markdown table.
///
/// For example, the second row of the table is the delimiter row.
///
/// |  a  |  b  |  c  |
/// | --- | --- | --- | <-- This is the alignment row
/// |     |     |     |
fn could_be_table(text: &str) -> bool {
    // I don't know why, but the regex still matches if there are escaped characters.
    // so I'm adding this check to prevent that.
    if text.contains('\\') {
        return false;
    }

    // If the text is just `-` and whitespace it's not a valid delimiter row
    if text
        .trim_matches(|c: char| c.is_whitespace() || c == '-')
        .is_empty()
    {
        return false;
    }

    // FIXME(ytmimi) when we're ready to bump the MSRV to 1.100.0
    // we can try using `trim_prefix` and `trim_suffix`.
    //
    // To simplify the regex check we're going to remove all the pipes.
    let text = text.trim();
    let text = text.strip_prefix('|').unwrap_or(text);
    let text = text.strip_suffix('|').unwrap_or(text);

    if text.is_empty() {
        return false;
    }

    text.split('|')
        .all(|cell| TABLE_DELIMITER_ROW_REGEX.is_match_at(cell.trim(), 0))
}

#[test]
fn test_could_be_table_delimeter_row() {
    let expected_matches = &[
        "|-- - --- - |- -|- - -|  -",
        "-|--|---|-",
        ":-|:--|:---|:-",
        "-:|--:|:---|-:",
        ":-|--|---|-:",
        "-:|--|---|:-",
        ":-:|:--:|:---:|:-:",
        "-|-",
        ":-|:-",
        "-:|-:",
        ":-|-:",
        "-:|:-",
        ":-:|:-:",
        "|-|",
        "|:-|",
        "|-:|",
        "|:-:|",
        "|-",
        "|:-",
        "|-:",
        "|:-:",
        "-|",
        ":-|",
        "-:|",
        ":-:|",
        "-|  -   ",
        "-|  -",
        ":-|  :-",
        "-:|  -:",
        "-:|  :-",
        ":-:|  :-:",
    ];

    for line in expected_matches {
        assert!(could_be_table(line))
    }
}

#[test]
fn test_is_not_table_delimeter_row() {
    let expected_rejections = &[
        r"|\-|",
        "|",
        "-",
        "- - - - - -",
        "|    ",
        " -  ",
        "some text",
        "|:--: - --- - |- -|- - -|  -",
    ];

    for line in expected_rejections {
        assert!(!could_be_table(line))
    }
}
