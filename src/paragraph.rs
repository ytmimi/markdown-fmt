use crate::escape::needs_escape;
use crate::writer::WriteEvent;

use pulldown_cmark::Event;
use regex::Regex;
use std::fmt::Write;
use std::sync::OnceLock;
use textwrap::Options as TextWrapOptions;

const MARKDOWN_HARD_BREAK: &str = "  \n";

/// A buffer where we write text
#[derive(Debug, PartialEq)]
pub(super) struct Paragraph {
    buffer: String,
    max_width: Option<usize>,
    should_reflow_text: bool,
}

impl WriteEvent<'_> for Paragraph {
    fn write_event_str(&mut self, e: &Event<'_>, s: &str) -> std::fmt::Result {
        // We should only need to escape `Event::Text`, and multi-line `Event::InlineHtml` and
        // `Event::Code` since they might contain characters that look like other Markdown
        // constructs, but they're really just text.
        if !matches!(e, Event::Text(_) | Event::InlineHtml(_) | Event::Code(_)) || s.is_empty() {
            return self.write_str(s);
        }

        // FIXME(ytmimi) I'm adding alot of checks here. They mostly duplicate what's defined
        // in `needs_escape`, but only apply in certain scenarios. There's probably a much
        // better way to handle this.

        // Prevent the next pass of the parser from accidentaly interpreting a table
        // without a leading |
        if self.buffer.ends_with('\n')
            && self.buffer.lines().last().is_some_and(|l| l.contains('|'))
            && could_be_table(s)
        {
            self.buffer.push('\\');
        }

        // Prevent the next pass from ignoring the hard break or misinterpreting `s`
        // as something other than text in a paragraph
        if self.buffer.ends_with(MARKDOWN_HARD_BREAK) && needs_escape(s) {
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
    static TABLE_ALIGNMENT_ROW_REGEX: OnceLock<Regex> = OnceLock::new();
    TABLE_ALIGNMENT_ROW_REGEX
        .get_or_init(|| {
            Regex::new(r"^((\|(\s?-+\s?)+)+\|?)+|((\s?-+\s?)+\|)+((\s?-+\s?))?$")
                .expect("valid regex")
        })
        .is_match_at(text, 0)
}

#[test]
fn test_could_be_table() {
    let expected_matches = &[
        "|-- - --- - |- -|- - -|  -",
        "-|--|---|-",
        "-|-",
        "|-|",
        "|-",
        "-|",
    ];

    for line in expected_matches {
        assert!(could_be_table(line))
    }

    let expected_rejections = &[
        r"|\-|",
        "|",
        "-",
        "- - - - - -",
        "|    ",
        " -  ",
        "some text",
    ];

    for line in expected_rejections {
        assert!(!could_be_table(line))
    }
}
