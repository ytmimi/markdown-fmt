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
