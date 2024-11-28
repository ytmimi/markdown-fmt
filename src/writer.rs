use std::fmt::Write;

use crate::footnote::FootnoteDefinition;
use crate::header::Header;
use crate::links::LinkWriter;
use crate::paragraph::Paragraph;
use crate::table::TableState;
use pulldown_cmark::CodeBlockKind;

#[derive(Debug, PartialEq)]
pub(super) enum MarkdownWriter<'i> {
    CodeBlock((String, CodeBlockKind<'i>)),
    Header(Header<'i>),
    FootnoteDefinition(FootnoteDefinition),
    Paragraph(Paragraph),
    Table(TableState<'i>),
    Link(LinkWriter),
}

impl MarkdownWriter<'_> {
    pub(super) fn is_empty(&self) -> bool {
        match self {
            Self::CodeBlock((c, _)) => c.is_empty(),
            Self::FootnoteDefinition(f) => f.is_empty(),
            Self::Header(h) => h.is_empty(),
            Self::Paragraph(p) => p.is_empty(),
            Self::Table(t) => t.is_empty(),
            Self::Link(l) => l.is_empty(),
        }
    }
}

impl std::fmt::Write for MarkdownWriter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self {
            Self::CodeBlock((c, _)) => c.write_str(s),
            Self::FootnoteDefinition(f) => f.write_str(s),
            Self::Header(h) => h.write_str(s),
            Self::Paragraph(p) => p.write_str(s),
            Self::Table(t) => t.write_str(s),
            Self::Link(l) => l.write_str(s),
        }
    }
}

impl WriteEvent<'_> for MarkdownWriter<'_> {
    fn write_event_str(&mut self, e: &pulldown_cmark::Event<'_>, s: &str) -> std::fmt::Result {
        match self {
            // Only need to implement this for `Self::Paragraph` right now to
            // give more context about when to escape text.
            Self::Paragraph(p) => p.write_event_str(e, s),
            _ => self.write_str(s),
        }
    }
}

impl<'i> From<Header<'i>> for MarkdownWriter<'i> {
    fn from(value: Header<'i>) -> Self {
        Self::Header(value)
    }
}

impl From<FootnoteDefinition> for MarkdownWriter<'_> {
    fn from(value: FootnoteDefinition) -> Self {
        Self::FootnoteDefinition(value)
    }
}

impl From<Paragraph> for MarkdownWriter<'_> {
    fn from(value: Paragraph) -> Self {
        Self::Paragraph(value)
    }
}

impl<'i> From<TableState<'i>> for MarkdownWriter<'i> {
    fn from(value: TableState<'i>) -> Self {
        Self::Table(value)
    }
}

impl From<LinkWriter> for MarkdownWriter<'_> {
    fn from(value: LinkWriter) -> Self {
        Self::Link(value)
    }
}

/// An extension to [std::fmt::Write], which passes context about the [pulldown_cmark::Event]
/// to the underlying writer.
pub(crate) trait WriteEvent<'i>: std::fmt::Write {
    /// Write a &str with additional context about which event is being written.
    fn write_event_str(&mut self, e: &pulldown_cmark::Event<'i>, s: &str) -> std::fmt::Result;

    /// Write a [`std::fmt::Arguments`] with additional context about which event is being written.
    fn write_event_fmt(
        &mut self,
        e: &pulldown_cmark::Event<'i>,
        args: std::fmt::Arguments<'_>,
    ) -> std::fmt::Result {
        if let Some(s) = args.as_str() {
            self.write_event_str(e, s)
        } else {
            self.write_event_str(e, &args.to_string())
        }
    }
}

impl WriteEvent<'_> for std::string::String {
    fn write_event_str(&mut self, _e: &pulldown_cmark::Event<'_>, s: &str) -> std::fmt::Result {
        self.write_str(s)
    }
}

/// Like the [write!] macro from the standard library, but also passes along context
/// about which [Event](pulldown_cmark::Event) is being written to the underlying writer.
///
/// **Note**, writers must implement [WriteEvent].
macro_rules! write_event {
    ($writer:expr, $event:expr, $($arg:tt)*) => {
        $writer.write_event_fmt($event, format_args!($($arg)*))
    };
}

pub(crate) use write_event;

/// Like the [writeln!] macro from the standard library, but also passes along context
/// about which [Event](pulldown_cmark::Event) is being written to the underlying writer.
///
/// **Note**, writers must implement [WriteEvent].
macro_rules! writeln_event {
    ($writer:expr, $event:expr $(,)?) => {
        write_event!($writer, $event, "\n")
    };
    ($writer:expr, $event:expr, $($arg:tt)*) => {
        write_event!($writer, $event, $($arg)*).and_then(|_| write_event!($writer, $event, "\n"))
    }
}

pub(crate) use writeln_event;
