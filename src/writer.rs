use std::fmt::Write;

use crate::definition_list::DefinitionListTitle;
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
    DefinitionListTitle(DefinitionListTitle),
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
            Self::DefinitionListTitle(d) => d.is_empty(),
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
            Self::DefinitionListTitle(d) => d.write_str(s),
        }
    }
}

impl WriteContext<'_> for MarkdownWriter<'_> {
    fn write_context_str(&mut self, ctx: MarkdownContext<'_, '_>, s: &str) -> std::fmt::Result {
        match self {
            Self::Paragraph(p) => p.write_context_str(ctx, s),
            Self::Header(h) => h.write_context_str(ctx, s),
            Self::DefinitionListTitle(d) => d.write_context_str(ctx, s),
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

impl From<DefinitionListTitle> for MarkdownWriter<'_> {
    fn from(value: DefinitionListTitle) -> Self {
        Self::DefinitionListTitle(value)
    }
}

/// An extension to [std::fmt::Write], which passes context about the [pulldown_cmark::Event]
/// or [pulldown_cmark::Tag] to the underlying writer.
pub(crate) trait WriteContext<'i>: std::fmt::Write {
    /// Write a &str with additional [context](WriteContext) about which
    /// [pulldown_cmark::Event] or [pulldown_cmark::Tag].
    fn write_context_str(&mut self, ctx: MarkdownContext<'_, 'i>, s: &str) -> std::fmt::Result;

    /// Write a &str with additional context about which [pulldown_cmark::Event] is being written.
    fn write_event_str(&mut self, e: &pulldown_cmark::Event<'i>, s: &str) -> std::fmt::Result {
        self.write_context_str(e.into(), s)
    }

    /// Write a &str with additional context about which [pulldown_cmark::Tag] is being written.
    fn write_tag_str(&mut self, t: &pulldown_cmark::Tag<'i>, s: &str) -> std::fmt::Result {
        self.write_context_str(t.into(), s)
    }

    /// Write a [`std::fmt::Arguments`] with additional [context](WriteContext) about which event is
    /// being written.
    fn write_context_fmt(
        &mut self,
        ctx: MarkdownContext<'_, 'i>,
        args: std::fmt::Arguments<'_>,
    ) -> std::fmt::Result {
        if let Some(s) = args.as_str() {
            self.write_context_str(ctx, s)
        } else {
            self.write_context_str(ctx, &args.to_string())
        }
    }

    /// Write a [`std::fmt::Arguments`] with additional context about which [pulldown_cmark::Event]
    /// is being written.
    #[allow(dead_code)]
    fn write_event_fmt(
        &mut self,
        e: &pulldown_cmark::Event<'i>,
        args: std::fmt::Arguments<'_>,
    ) -> std::fmt::Result {
        self.write_context_fmt(e.into(), args)
    }

    /// Write a [`std::fmt::Arguments`] with additional context about which [pulldown_cmark::Tag]
    /// is being written.
    #[allow(dead_code)]
    fn write_tag_fmt(
        &mut self,
        t: &pulldown_cmark::Tag<'i>,
        args: std::fmt::Arguments<'_>,
    ) -> std::fmt::Result {
        self.write_context_fmt(t.into(), args)
    }
}

/// Context about the [pulldown_cmark::Evet] or [pulldown_cmark::Tag] that's being written.
#[derive(Debug, Clone)]
pub(crate) enum MarkdownContext<'a, 'i> {
    Event(&'a pulldown_cmark::Event<'i>),
    Tag(&'a pulldown_cmark::Tag<'i>),
    Escape,
}

impl<'a, 'i> From<&'a pulldown_cmark::Event<'i>> for MarkdownContext<'a, 'i> {
    fn from(value: &'a pulldown_cmark::Event<'i>) -> Self {
        Self::Event(value)
    }
}

impl<'a, 'i> From<&'a pulldown_cmark::Tag<'i>> for MarkdownContext<'a, 'i> {
    fn from(value: &'a pulldown_cmark::Tag<'i>) -> Self {
        Self::Tag(value)
    }
}

impl WriteContext<'_> for std::string::String {
    fn write_context_str(&mut self, _ctx: MarkdownContext<'_, '_>, s: &str) -> std::fmt::Result {
        self.write_str(s)
    }
}

/// Like the [write!] macro from the standard library, but also passes along context
/// about which [Event](pulldown_cmark::Event) is being written to the underlying writer.
///
/// **Note**, writers must implement [WriteEvent].
macro_rules! write_context {
    ($writer:expr, Escape, $($arg:tt)*) => {
        $writer.write_context_fmt($crate::writer::MarkdownContext::Escape, format_args!($($arg)*))
    };
    ($writer:expr, $ctx:expr, $($arg:tt)*) => {
        $writer.write_context_fmt($ctx.into(), format_args!($($arg)*))
    };
}

pub(crate) use write_context;

/// Like the [writeln!] macro from the standard library, but also passes along context
/// about which [Event](pulldown_cmark::Event) is being written to the underlying writer.
///
/// **Note**, writers must implement [WriteEvent].
macro_rules! writeln_context {
    ($writer:expr, Escape, $($arg:tt)*) => {{
        let ctx = $crate::writer::MarkdownContext::Escape;
        $writer.write_context_fmt(ctx.clone(), format_args!($($arg)*))
            .and_then(|_| $writer.write_context_str(ctx.clone(), "\n"))
    }};
    ($writer:expr, $ctx:expr $(,)?) => {
        write_context!($writer, $ctx, "\n")
    };
    ($writer:expr, $ctx:expr, $($arg:tt)*) => {
        write_context!($writer, $ctx, $($arg)*).and_then(|_| write_context!($writer, $ctx, "\n"))
    }
}

pub(crate) use writeln_context;
