use std::fmt::Write;

static FOOTNOTE_INDENTATION: &str = "    ";

/// A buffer where we write footnote definition text
#[derive(Debug, PartialEq)]
pub(crate) struct FootnoteDefinition {
    buffer: String,
}

impl Write for FootnoteDefinition {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl FootnoteDefinition {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    /// Get the indentation for footnote definitions
    pub(super) fn indentation() -> &'static str {
        FOOTNOTE_INDENTATION
    }

    /// Check if the internal buffer is empty
    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Consume Self and return the formatted buffer
    pub(super) fn into_buffer(self) -> String {
        self.buffer
    }
}
