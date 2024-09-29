use std::borrow::Cow;
use std::fmt::Write;

static FOOTNOTE_INDENTATION: &str = "    ";

/// A buffer where we write footnote definition text
#[derive(Debug, PartialEq)]
pub(crate) struct FootnoteDefinition {
    buffer: String,
    indentation: Vec<Cow<'static, str>>,
}

impl Write for FootnoteDefinition {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl FootnoteDefinition {
    pub(super) fn new(indentation: Vec<Cow<'static, str>>, capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            indentation,
        }
    }

    /// Check if the internal buffer is empty
    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Consume Self and return the formatted buffer
    pub(super) fn into_parts(self) -> (String, Vec<Cow<'static, str>>, Cow<'static, str>) {
        (self.buffer, self.indentation, FOOTNOTE_INDENTATION.into())
    }
}
