use std::fmt::Write;
use textwrap::Options as TextWrapOptions;

const MARKDOWN_HARD_BREAK: &str = "  \n";

/// A buffer where we write text
pub(super) struct Paragraph {
    buffer: String,
    max_width: Option<usize>,
}

impl Write for Paragraph {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let is_hard_break = |s: &str| -> bool {
            // Hard breaks can have any amount of leading whitesace followed by a newline
            s.strip_prefix("  ")
                .is_some_and(|maybe_hard_break| maybe_hard_break.trim_start_matches(' ').eq("\n"))
        };

        if self.max_width.is_some() && is_hard_break(s) {
            self.buffer.push_str(MARKDOWN_HARD_BREAK);
            return Ok(());
        }

        if self.max_width.is_some() && s.trim().is_empty() {
            // If the user configured the max_width then push a space so we can reflow text
            self.buffer.push(' ');
        } else {
            self.buffer.push_str(s);
        }

        Ok(())
    }
}

impl Paragraph {
    pub(super) fn new(max_width: Option<usize>, capacity: usize) -> Self {
        Self {
            max_width,
            buffer: String::with_capacity(capacity),
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
            let wrapped_text = textwrap::fill(text, wrap_options.clone());
            output_buffer.push_str(&wrapped_text);
            if has_next {
                output_buffer.push_str(MARKDOWN_HARD_BREAK);
            }
        }

        output_buffer
    }
}
