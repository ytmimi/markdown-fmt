use crate::escape::needs_escape;
use crate::writer::{MarkdownContext, WriteContext};
use pulldown_cmark::Event;
use std::fmt::Write;

/// A buffer for writing Definition lists
#[derive(Debug, PartialEq)]
pub(super) struct DefinitionListTitle {
    buffer: String,
}

impl<'i> WriteContext<'i> for DefinitionListTitle {
    fn write_context_str(&mut self, ctx: MarkdownContext<'_, 'i>, s: &str) -> std::fmt::Result {
        let ctx_does_not_need_escape = !matches!(
            ctx,
            MarkdownContext::Event(Event::Text(_) | Event::InlineHtml(_) | Event::Code(_))
        );

        if ctx_does_not_need_escape || !self.buffer.ends_with('\n') {
            return self.write_str(s);
        }

        match needs_escape(s) {
            Some(escape_kind) if escape_kind.multi_character_escape() => {
                let marker = escape_kind.marker();
                for c in s.chars() {
                    if marker == c {
                        self.write_char('\\')?;
                    }
                    self.write_char(c)?;
                }
                Ok(())
            }
            Some(_) => {
                self.write_char('\\')?;
                self.write_str(s)
            }
            None => self.write_str(s),
        }
    }
}

impl std::fmt::Write for DefinitionListTitle {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }
}

impl DefinitionListTitle {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub(super) fn into_buffer(self) -> String {
        self.buffer
    }
}
