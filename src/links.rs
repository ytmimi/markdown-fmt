use super::formatter::FormatState;
use crate::utils::{is_char_esacped, sequence_ends_on_escape};
use pulldown_cmark::Event;
use std::borrow::Cow;
use std::fmt::Write;

/// Rewrites the content of all [LinkType](pulldown_cmark::LinkType) Events.
#[derive(Debug, PartialEq)]
pub(crate) struct LinkWriter {
    buffer: String,
}

impl Write for LinkWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.is_empty() {
            // While the buffer is empty trim leading whitespace
            let s = s.trim_start();
            if s.starts_with('^') {
                self.buffer.push('\\');
            }
            self.buffer.push_str(s.trim_start());
            return Ok(());
        }

        self.buffer.push_str(s);
        Ok(())
    }
}

impl LinkWriter {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub(crate) fn into_buffer(mut self) -> String {
        // Remove any trailing whitespace from the buffer
        while self.buffer.ends_with(char::is_whitespace) {
            self.buffer.pop();
        }

        self.buffer
    }
}

impl<'i, I> FormatState<'i, '_, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    pub(super) fn write_inline_link<S: AsRef<str>>(
        &mut self,
        url: &str,
        title: Option<(S, char)>,
    ) -> std::fmt::Result {
        let url = format_link_url(url, false);
        match title {
            Some((title, ')')) => write!(self, r#"]({url} ({}))"#, title.as_ref())?,
            Some((title, quote)) => write!(self, r#"]({url} {quote}{}{quote})"#, title.as_ref())?,
            None => write!(self, "]({url})")?,
        }
        Ok(())
    }
}

pub(crate) fn format_link_url(url: &str, wrap_empty_urls: bool) -> Cow<'_, str> {
    if wrap_empty_urls && url.is_empty() {
        Cow::from("<>")
    } else if !url.starts_with('<') && !url.ends_with('>') && url.contains(' ')
        || !balanced_parens(url)
    {
        // https://spec.commonmark.org/0.30/#link-destination
        Cow::from(format!("<{url}>"))
    } else {
        url.into()
    }
}

/// Check if the parens are balanced
fn balanced_parens(url: &str) -> bool {
    let mut stack = vec![];
    let mut was_last_escape = false;

    for b in url.bytes() {
        if !was_last_escape && b == b'(' {
            stack.push(b);
            continue;
        }

        if !was_last_escape && b == b')' {
            if let Some(top) = stack.last() {
                if *top != b'(' {
                    return false;
                }
                stack.pop();
            } else {
                return false;
            }
        }
        was_last_escape = b == b'\\';
    }
    stack.is_empty()
}

/// Search for enclosing balanced brackets
fn find_text_within_last_set_of_balance_bracket(
    label: &str,
    opener: u8,
    closer: u8,
    halt_condition: Option<fn(u8) -> bool>,
) -> (usize, usize) {
    let mut stack = vec![];
    let mut was_last_escape = false;

    let mut start = 0;
    let mut end = label.len();

    let mut bytes = label.bytes().enumerate().peekable();

    while let Some((index, byte)) = bytes.next() {
        if !was_last_escape && byte == opener {
            stack.push(index)
        }

        if !was_last_escape && byte == closer {
            if let Some(start_index) = stack.pop() {
                start = start_index;
                end = index;
            }

            if stack.is_empty() && halt_condition.is_some() {
                match (bytes.peek(), halt_condition) {
                    (Some((_, byte)), Some(halt_condition)) if halt_condition(*byte) => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        was_last_escape = byte == b'\\'
    }
    (start, end + 1)
}

/// Reference links are expected to be well formed:
/// [foo][bar] -> bar
/// [link \[bar][ref] -> ref
pub(super) fn find_reference_link_label(input: &str) -> &str {
    let (start, end) = find_text_within_last_set_of_balance_bracket(input, b'[', b']', None);
    // +1 to move past '['
    // -1 to move before ']'
    input[start + 1..end - 1].trim()
}

/// Inline links are expected to be well formed:
/// [link](/uri) -> '/uri'
/// [link](</my uri>) -> '/my uri'
pub(super) fn find_inline_url_and_title(input: &str) -> Option<(String, Option<(String, char)>)> {
    let (_, end) =
        find_text_within_last_set_of_balance_bracket(input, b'[', b']', Some(|b| b == b'('));
    // +1 to move past '('
    // -1 to move before ')'
    let inline_url = input[end + 1..input.len() - 1].trim();
    if inline_url.is_empty() {
        return Some((String::new(), None));
    }

    split_inline_url_from_title(inline_url, inline_url.ends_with(['"', '\'', ')']))
}

// The link must have a title if we're calling this
fn link_title_start(link: &[u8]) -> usize {
    let last = *link.last().expect("links titles must have quotes");
    let opener = if last == b')' { b'(' } else { last };

    // offset by 2 to skip triling quote
    let mut index = link.len() - 2;
    while index.saturating_sub(1) != 0 {
        if link[index] == opener && link[index - 1] != b'\\' {
            return index;
        }
        index -= 1;
    }

    // Odd case where a title is in the place of a url
    //https://spec.commonmark.org/0.30/#example-503
    0
}

fn trim_angle_brackes(url: &str) -> &str {
    if url.starts_with('<') && url.ends_with('>') {
        url[1..url.len() - 1].trim()
    } else {
        url.trim()
    }
}

fn split_inline_url_from_title(
    input: &str,
    has_title: bool,
) -> Option<(String, Option<(String, char)>)> {
    // If both link destination and link title are present, they must be separated by spaces, tabs,
    // and up to one line ending.
    let has_space = input.contains(char::is_whitespace);
    let only_link = !has_space && has_title;
    let link_start = link_title_start(input.as_bytes());
    if only_link || !has_title || link_start == 0 {
        return Some((trim_angle_brackes(input).to_string(), None));
    }

    let (mut url, mut title_with_quotes) = input.split_at(link_start);

    url = url.trim();

    title_with_quotes = title_with_quotes.trim();

    // Remove the wrapping quotes from the title
    let quote = title_with_quotes
        .bytes()
        .last()
        .expect("url title has a quote") as char;
    let title = &title_with_quotes[1..title_with_quotes.len() - 1];

    Some((
        trim_angle_brackes(url).to_string(),
        Some((title.to_string(), quote)),
    ))
}

struct LinkReferenceDefinitionBuilder<'a> {
    label: Option<LinkLines<'a>>,
    destination: Option<(LinkDestination<'a>, std::ops::Range<usize>)>,
    title: Option<LinkTitle<'a>>,
}

impl<'a> LinkReferenceDefinitionBuilder<'a> {
    fn new() -> Self {
        Self {
            label: None,
            destination: None,
            title: None,
        }
    }

    fn set_label(&mut self, label: Cow<'a, str>, range: std::ops::Range<usize>, offset: usize) {
        let mut label_parts = self.label.take().unwrap_or(LinkLines::new());
        let offset_range = (offset + range.start)..(offset + range.end);
        if label_parts.is_empty() && label.starts_with('^') {
            let mut new_label = String::with_capacity(label.len() + 1);
            new_label.push('\\');
            new_label.push_str(&label);
            label_parts.push((new_label.into(), offset_range));
        } else {
            label_parts.push((label, offset_range));
        }
        self.label = Some(label_parts);
    }

    fn set_url(
        &mut self,
        destination: LinkDestination<'a>,
        range: std::ops::Range<usize>,
        offset: usize,
    ) {
        let offset_range = (offset + range.start)..(offset + range.end);
        self.destination = Some((destination, offset_range));
    }

    fn set_title(
        &mut self,
        kind: TitleMarker,
        value: Cow<'a, str>,
        range: std::ops::Range<usize>,
        offset: usize,
    ) {
        let offset_range = (offset + range.start)..(offset + range.end);
        if let Some(title) = self.title.as_mut() {
            title.push((value, offset_range))
        } else {
            let title = LinkTitle::new(kind, (value, offset_range));
            self.title = Some(title);
        }
    }

    fn has_title(&self) -> bool {
        self.title.is_some()
    }

    fn remove_false_title(&mut self) {
        tracing::trace!("Removing False Title");
        self.title = None;
    }

    fn build(self) -> Option<LinkReferenceDefinition<'a>> {
        Some(LinkReferenceDefinition {
            label: self.label?,
            destination: self.destination?,
            title: self.title,
        })
    }
}

/// [Link Reference Definition]
///
/// For example, here's a reference defintion.
/// ```markdown
/// [label]: /destination "title"
/// ```
/// [link reference definition]: https://spec.commonmark.org/0.31.2/#link-reference-definition
#[derive(Debug, PartialEq, Eq)]
pub(super) struct LinkReferenceDefinition<'a> {
    label: LinkLines<'a>,
    destination: (LinkDestination<'a>, std::ops::Range<usize>),
    title: Option<LinkTitle<'a>>,
}

impl LinkReferenceDefinition<'_> {
    pub(super) fn range(&self) -> std::ops::Range<usize> {
        let start = self.label.range().expect("we have a label").start;
        let end = if let Some(title) = self.title.as_ref() {
            title.range().expect("we have a title").end
        } else {
            self.destination.1.end
        };
        start..end
    }

    pub(super) fn write<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "[")?;
        self.label.write(writer)?;
        write!(writer, "]: ")?;
        self.destination.0.write(writer)?;
        if let Some(title) = self.title.as_ref() {
            write!(writer, " ")?;
            title.write(writer)?;
        }

        Ok(())
    }
}

/// [Link Destination]
///
/// [link destination]: https://spec.commonmark.org/0.31.2/#link-destination
#[derive(Debug, PartialEq, Eq)]
pub(super) enum LinkDestination<'a> {
    /// A link destination with brackets e.g. <some/url/>
    Bracketed(Cow<'a, str>),
    /// A nonempty sequence of characters
    Regular(Cow<'a, str>),
}

impl LinkDestination<'_> {
    fn write<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        match self {
            Self::Bracketed(text) => write!(writer, "<{text}>"),
            Self::Regular(text) => write!(writer, "{text}"),
        }
    }
}

/// [Link Title]
///
/// [link title]: https://spec.commonmark.org/0.31.2/#link-title
#[derive(Debug, PartialEq, Eq)]
pub(super) struct LinkTitle<'a> {
    kind: TitleMarker,
    value: LinkLines<'a>,
}

impl<'a> LinkTitle<'a> {
    fn new(kind: TitleMarker, value: (Cow<'a, str>, std::ops::Range<usize>)) -> Self {
        Self {
            kind,
            value: LinkLines::from(value),
        }
    }

    fn push(&mut self, value: (Cow<'a, str>, std::ops::Range<usize>)) {
        self.value.push(value)
    }

    fn range(&self) -> Option<std::ops::Range<usize>> {
        self.value.range()
    }

    fn write<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "{}", self.kind.opener())?;
        self.value.write(writer)?;
        write!(writer, "{}", self.kind.closer())
    }
}

/// Marker use to wrap the link title
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum TitleMarker {
    /// Double quoted title like `"title"`
    DoubleQuote,
    /// Single quoted title like `'title'`
    SingleQuote,
    /// Title wrapped in Parentheses like `(title)`
    Parentheses,
}

impl TitleMarker {
    fn opener(&self) -> char {
        match &self {
            Self::DoubleQuote => '"',
            Self::SingleQuote => '\'',
            Self::Parentheses => '(',
        }
    }

    fn closer(&self) -> char {
        match &self {
            Self::DoubleQuote => '"',
            Self::SingleQuote => '\'',
            Self::Parentheses => ')',
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct InvalidTitleMarker(char);

impl TryFrom<char> for TitleMarker {
    type Error = InvalidTitleMarker;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\'' => Ok(TitleMarker::SingleQuote),
            '"' => Ok(TitleMarker::DoubleQuote),
            '(' => Ok(TitleMarker::Parentheses),
            _ => Err(InvalidTitleMarker(value)),
        }
    }
}

/// Collection of snippets that make up a reference link definition
#[derive(Debug, PartialEq, Eq)]
pub(super) struct LinkLines<'a>(Vec<(Cow<'a, str>, std::ops::Range<usize>)>);

impl<'a> LinkLines<'a> {
    fn new() -> Self {
        LinkLines(Vec::new())
    }

    fn range(&self) -> Option<std::ops::Range<usize>> {
        let (_, first) = self.0.first()?;
        let (_, last) = self.0.last()?;
        Some(first.start..last.end)
    }

    fn push(&mut self, value: (Cow<'a, str>, std::ops::Range<usize>)) {
        self.0.push(value)
    }

    fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|(item, _)| item.as_ref())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn write<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        let mut buffer = String::new();
        // FIXME(ytmimi) probably should provide an option to allow multi-line lines.
        // right now everything gets formatted on a single line.
        let mut iter = self.iter().peekable();
        while let Some(text) = iter.next().map(|t| t.trim()) {
            if text.is_empty() {
                continue;
            }

            let is_last = iter.peek().is_none();
            if !is_last {
                write!(buffer, "{text} ")?;
            } else if sequence_ends_on_escape(text) {
                // Escape this so that the next formatting run doesn't
                // interpret the closing `]` as `\]`.
                write!(buffer, "{text}\\")?;
            } else {
                write!(buffer, "{text}")?;
            }
        }
        write!(writer, "{}", buffer.trim())
    }
}

impl<'a> From<(Cow<'a, str>, std::ops::Range<usize>)> for LinkLines<'a> {
    fn from(value: (Cow<'a, str>, std::ops::Range<usize>)) -> Self {
        LinkLines(vec![(value)])
    }
}

impl From<(&'static str, std::ops::Range<usize>)> for LinkLines<'_> {
    fn from(value: (&'static str, std::ops::Range<usize>)) -> Self {
        LinkLines(vec![(Cow::from(value.0), value.1)])
    }
}

impl<'a> From<Vec<(Cow<'a, str>, std::ops::Range<usize>)>> for LinkLines<'a> {
    fn from(value: Vec<(Cow<'a, str>, std::ops::Range<usize>)>) -> Self {
        LinkLines(value)
    }
}

pub fn parse_link_reference_definitions<'i>(
    input: &'i str,
    mut offset: usize,
) -> Vec<LinkReferenceDefinition<'i>> {
    let mut refernce_definitions = vec![];
    let mut input = input;

    while let Some((def, idx)) = parse_link_reference_definition(input, offset) {
        refernce_definitions.push(def);
        offset += idx;
        input = &input[idx..];
    }
    refernce_definitions
}

/// What part of the link reference definition are we currently trying to parse
#[derive(Debug)]
enum LinkParserPhase {
    /// Find the opening `[` in the label
    FindOpeningBracket,
    /// Parse the label inside the brackets `[]`
    Label,
    /// Find the colon after the label
    Colon,
    /// Find the start of the URL
    UrlStart,
    /// Find the url after the colon
    Url(char),
    /// Find the start of the title
    TitleStart,
    /// Find the optional title after the Url
    Title(TitleMarker),
    /// Eat indentation characters after a newline
    HandleNewline(MultiLinePhase),
}

#[derive(Debug, Clone, Copy)]
enum MultiLinePhase {
    Label,
    Title(TitleMarker),
}

/// Parse a single reference definition from
fn parse_link_reference_definition<'i>(
    input: &'i str,
    offset: usize,
) -> Option<(LinkReferenceDefinition<'i>, usize)> {
    let mut phase = LinkParserPhase::FindOpeningBracket;
    let mut builder = LinkReferenceDefinitionBuilder::new();
    let mut start = 0;
    let mut parsed_until = 0;
    let mut newline_count = 0;
    let mut is_escaped: bool = false;

    let mut iter = input.char_indices().peekable();

    while let Some((idx, c)) = iter.next() {
        tracing::trace!("c: {c:?}, phase: {phase:?} is_escaped: {is_escaped}");
        match phase {
            LinkParserPhase::FindOpeningBracket => {
                if c != '[' {
                    continue;
                }
                start = iter.peek().map(|(idx, _)| *idx)?;
                tracing::trace!("Transition to LinkParserPhase::Label");
                phase = LinkParserPhase::Label;
            }
            LinkParserPhase::Label => {
                if c == '\n' {
                    let label = Cow::from(&input[start..idx]);
                    builder.set_label(label, start..idx, offset);
                    // Handle the newline if the next char doesn't immediatly terminate the label
                    if iter.peek().is_some_and(|(_, c)| *c != ']') {
                        tracing::trace!(
                            "Transition to LinkParserPhase::HandleNewline(MultiLinePhase::Label)"
                        );
                        phase = LinkParserPhase::HandleNewline(MultiLinePhase::Label);
                    }
                    start = iter.peek().map(|(idx, _)| *idx)?;
                    continue;
                }
                if c != ']' || is_escaped {
                    is_escaped = is_char_esacped(c, is_escaped);
                    continue;
                }
                let label = Cow::from(&input[start..idx]);
                builder.set_label(label, start..idx, offset);
                tracing::trace!("Transition to LinkParserPhase::Colon");
                phase = LinkParserPhase::Colon;
                parsed_until = idx;
                is_escaped = false;
            }
            LinkParserPhase::Colon => {
                if c != ':' {
                    continue;
                }
                tracing::trace!("Transition to LinkParserPhase::UrlStart");
                phase = LinkParserPhase::UrlStart;
                parsed_until = idx;
            }
            LinkParserPhase::HandleNewline(next_phase) => {
                let next_is_important = match (next_phase, iter.peek()) {
                    (MultiLinePhase::Label, Some((_, ']'))) => true,
                    (MultiLinePhase::Title(marker), Some((_, c))) => {
                        *c == marker.opener() || *c == marker.closer()
                    }
                    _ => false,
                };
                // FIXME(ytmimi) Assuming that `>` indicates a blockquote, but maybe there's a
                // chance that its part of the title or label
                if !next_is_important && (c.is_whitespace() || c == '>') {
                    continue;
                }
                start = idx;
                match next_phase {
                    MultiLinePhase::Label => {
                        tracing::trace!("Transition to LinkParserPhase::Label");
                        phase = LinkParserPhase::Label
                    }
                    MultiLinePhase::Title(marker) => {
                        tracing::trace!("Transition to LinkParserPhase::Title(marker)");
                        phase = LinkParserPhase::Title(marker)
                    }
                }
            }
            LinkParserPhase::UrlStart => {
                // TODO(ytmimi) handle newlines in label. If we're in a nested context like
                // a block quote, then we need to potentially ignore leading `>` chars.
                //
                // For now, just assume we can eat all chars until we reach the start of the URL
                if c.is_whitespace() || c == '>' {
                    continue;
                }

                // It's possible that the link is a single character
                let is_last = iter.peek().is_none();
                if is_last {
                    let url = Cow::from(&input[idx..=idx]);
                    builder.set_url(LinkDestination::Regular(url), idx..idx, offset);
                    parsed_until = idx;
                    break;
                }

                // We're at the start of the URL
                start = idx;
                tracing::trace!("Transition to LinkParserPhase::Url(c)");
                phase = LinkParserPhase::Url(c);
                parsed_until = idx;
            }
            LinkParserPhase::Url(start_char) => {
                // Taking a look at the [link destination spec], I don't think we can have newlines
                // within the destination.
                // [link destination spec]: https://spec.commonmark.org/0.30/#link-destination
                match start_char {
                    '<' => {
                        if c != '>' || is_escaped {
                            is_escaped = is_char_esacped(c, is_escaped);
                            continue;
                        }

                        let url = Cow::from(&input[start + 1..idx]);
                        builder.set_url(LinkDestination::Bracketed(url), start..idx, offset);
                        tracing::trace!("Transition to LinkParserPhase::TitleStart");
                        phase = LinkParserPhase::TitleStart;
                        parsed_until = idx;
                    }
                    _ => {
                        if !c.is_whitespace() {
                            let is_last = iter.peek().is_none();
                            if is_last {
                                let url = Cow::from(&input[start..=idx]);
                                builder.set_url(LinkDestination::Regular(url), start..idx, offset);
                                parsed_until = idx;
                                break;
                            }
                            continue;
                        }

                        if c == '\n' {
                            newline_count += 1;
                        }

                        let url = Cow::from(&input[start..idx]);
                        builder.set_url(LinkDestination::Regular(url), start..idx, offset);
                        tracing::trace!("Transition to LinkParserPhase::TitleStart");
                        phase = LinkParserPhase::TitleStart;
                        parsed_until = idx;
                    }
                }
            }
            LinkParserPhase::TitleStart => {
                if c == '\n' {
                    newline_count += 1;
                    if newline_count > 1 {
                        // There can only be one newline between the end of the URL and the
                        // start of the title
                        parsed_until = idx;
                        break;
                    }
                }

                if c.is_whitespace() || c == '>' {
                    continue;
                }

                match TitleMarker::try_from(c) {
                    Ok(marker) => {
                        start = iter.peek().map(|(idx, _)| *idx)?;
                        tracing::trace!("Transition to LinkParserPhase::Title(marker)");
                        phase = LinkParserPhase::Title(marker);
                        parsed_until = idx;
                    }
                    Err(_) => {
                        // If we don't have a title opener then this isn't a title
                        parsed_until = idx;
                        break;
                    }
                }
            }
            LinkParserPhase::Title(marker) => {
                if c == '\n' {
                    let label = Cow::from(&input[start..idx]);
                    builder.set_title(marker, label, start..idx, offset);
                    // Handle the newline if the next char doesn't immediatly terminate the title
                    if iter.peek().is_some_and(|(_, c)| *c != marker.closer()) {
                        tracing::trace!(
                            "Transition to LinkParserPhase::HandleNewline(MultiLinePhase::Title)"
                        );
                        phase = LinkParserPhase::HandleNewline(MultiLinePhase::Title(marker));
                    }
                    start = iter.peek().map(|(idx, _)| *idx)?;
                    continue;
                }
                if c != marker.closer() || is_escaped {
                    is_escaped = is_char_esacped(c, is_escaped);
                    continue;
                }

                let title = Cow::from(&input[start..idx]);
                builder.set_title(marker, title, start..idx, offset);
                parsed_until = idx;
                tracing::trace!("Done Parsing Link Definition");
                break;
            }
        }
    }

    // Titles can't be followed by any non whitespace chars on their line
    if builder.has_title() {
        // Check if there are any non-whitespace characters that come after the title
        for (idx, c) in iter {
            if c == '\n' {
                // We only need to check up to the newline
                // +1 so that we start after the newline. Also, +1 is fine since '\n' is ascii
                parsed_until = idx + 1;
                break;
            }

            if !c.is_whitespace() {
                builder.remove_false_title();
                parsed_until = idx;
                break;
            }
        }
    }

    builder.build().map(|def| (def, parsed_until))
}

#[cfg(test)]
mod test {
    use super::*;

    fn cmp_single_line(rhs: LinkLines<'_>, line: &str) -> bool {
        if rhs.0.len() > 1 {
            return false;
        }

        matches!(rhs.0.first(), Some((l, _)) if l == line)
    }

    macro_rules! check_parsed_link_reference_definition {
        (definition:$definition:literal, label:$label:literal, url:$url:expr,) => {
            check_parsed_link_reference_definition! {
                check
                definition: $definition,
                label: $label,
                url: $url,
                title: Option::<&str>::None,
            }
        };
        (
            definition:$definition:literal,
            label:$label:literal,
            url:$url:expr,
            title:$title:expr,
        ) => {
            check_parsed_link_reference_definition! {
                check
                definition: $definition,
                label: $label,
                url: $url,
                title: Some($title),
            }
        };
        (
            check
            definition:$definition:literal,
            label:$label:literal,
            url:$url:expr,
            title:$title:expr,
        ) => {
            let result = parse_link_reference_definition($definition, 0).unwrap().0;
            assert!(cmp_single_line(result.label, $label));
            assert_eq!(result.destination.0, $url);
            if $title.is_some() {
                assert!(cmp_single_line(
                    result.title.unwrap().value,
                    $title.unwrap()
                ));
            } else {
                assert!(result.title.is_none());
            }
        };
    }

    #[test]
    fn test_parse_link_reference_definition() {
        check_parsed_link_reference_definition! {
            definition: "[foo-one]: foo-url 'single-quote-title'",
            label: "foo-one",
            url: LinkDestination::Regular("foo-url".into()),
            title: "single-quote-title",
        }

        check_parsed_link_reference_definition! {
            definition: "[foo-two]: <foo-url> 'single-quote-title'",
            label: "foo-two",
            url: LinkDestination::Bracketed("foo-url".into()),
            title: "single-quote-title",
        }

        check_parsed_link_reference_definition! {
            definition: "[foo-three]: no-title",
            label: "foo-three",
            url: LinkDestination::Regular(Cow::from("no-title")),
        }

        check_parsed_link_reference_definition! {
            definition: r#"[bar-one]: bar-url "double-quote-title""#,
            label: "bar-one",
            url: LinkDestination::Regular("bar-url".into()),
            title: "double-quote-title",
        }

        check_parsed_link_reference_definition! {
            definition: r#"[bar-two]: <bar-url> "double-quote-title""#,
            label: "bar-two",
            url: LinkDestination::Bracketed("bar-url".into()),
            title: "double-quote-title",
        }

        check_parsed_link_reference_definition! {
            definition: r#"[bar-three]: no-title"#,
            label: "bar-three",
            url: LinkDestination::Regular("no-title".into()),
        }

        check_parsed_link_reference_definition! {
            definition:  "[baz-one]: baz-url (paren-title)",
            label: "baz-one",
            url: LinkDestination::Regular("baz-url".into()),
            title: "paren-title",
        }

        check_parsed_link_reference_definition! {
            definition: "[baz-two]: <baz-url> (paren-title)",
            label: "baz-two",
            url: LinkDestination::Bracketed("baz-url".into()),
            title: "paren-title",
        }

        check_parsed_link_reference_definition! {
            definition: "[baz-three]: no-title",
            label: "baz-three",
            url: LinkDestination::Regular("no-title".into()),
        }

        check_parsed_link_reference_definition! {
            definition: "[empty-url]: <> 'single-quote-title'",
            label: "empty-url",
            url: LinkDestination::Bracketed("".into()),
            title: "single-quote-title",
        }

        check_parsed_link_reference_definition! {
            definition: "[empty-url]: <> 'single-quote-title' <- not a title bc of this extra text",
            label: "empty-url",
            url: LinkDestination::Bracketed("".into()),
        }

        check_parsed_link_reference_definition! {
            definition: "[.]:[ ",
            label: ".",
            url: LinkDestination::Regular("[".into()),
        }

        check_parsed_link_reference_definition! {
            definition: "[.]: [",
            label: ".",
            url: LinkDestination::Regular("[".into()),
        }

        check_parsed_link_reference_definition! {
            definition: "[.]:[]:[]",
            label: ".",
            url: LinkDestination::Regular("[]:[]".into()),
        }

        check_parsed_link_reference_definition! {
            definition: "[\\ ]:]",
            label: "\\ ",
            url: LinkDestination::Regular("]".into()),
        }
    }

    #[test]
    fn test_parse_multiple_link_reference_definitions() {
        let definition = r#"
[foo]: /foo
[bar]: /bar (title)
[baz]: /baz ''
"#;
        let result = parse_link_reference_definitions(definition, 0);
        let expected = vec![
            LinkReferenceDefinition {
                label: ("foo", 2..5).into(),
                destination: (LinkDestination::Regular("/foo".into()), 8..12),
                title: None,
            },
            LinkReferenceDefinition {
                label: ("bar", 14..17).into(),
                destination: (LinkDestination::Regular("/bar".into()), 20..24),
                title: Some(LinkTitle {
                    kind: TitleMarker::Parentheses,
                    value: ("title", 26..31).into(),
                }),
            },
            LinkReferenceDefinition {
                label: ("baz", 34..37).into(),
                destination: (LinkDestination::Regular("/baz".into()), 40..44),
                title: Some(LinkTitle {
                    kind: TitleMarker::SingleQuote,
                    value: ("", 46..46).into(),
                }),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_multi_line_link_reference_definitions() {
        let definition = r#"
[foo]:
   /foo
[bar]:
   /bar
   (title)
[baz]:
   /baz
   ''
"#;

        let result = parse_link_reference_definitions(definition, 0);
        let expected = vec![
            LinkReferenceDefinition {
                label: ("foo", 2..5).into(),
                destination: (LinkDestination::Regular("/foo".into()), 11..15),
                title: None,
            },
            LinkReferenceDefinition {
                label: ("bar", 17..20).into(),
                destination: (LinkDestination::Regular("/bar".into()), 26..30),
                title: Some(LinkTitle {
                    kind: TitleMarker::Parentheses,
                    value: ("title", 35..40).into(),
                }),
            },
            LinkReferenceDefinition {
                label: ("baz", 43..46).into(),
                destination: (LinkDestination::Regular("/baz".into()), 52..56),
                title: Some(LinkTitle {
                    kind: TitleMarker::SingleQuote,
                    value: ("", 61..61).into(),
                }),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_nested_multi_line_link_reference_definitions() {
        let definition = r#">
>[foo]:
>   /foo
"#;

        let result = parse_link_reference_definitions(definition, 0);
        let expected = vec![LinkReferenceDefinition {
            label: ("foo", 4..7).into(),
            destination: (LinkDestination::Regular("/foo".into()), 14..18),
            title: None,
        }];
        assert_eq!(result, expected);
    }

    #[test]
    fn with_multi_line_label() {
        let definition = r#"
[foo
 bar
]:
   /foo-bar
[
    oof
    rab
]: /oof-rab
"this is a title"
"#;

        let result = parse_link_reference_definitions(definition, 0);
        #[rustfmt::skip]
        let expected = vec![
            LinkReferenceDefinition {
                label: vec![
                    (Cow::from("foo"), 2..5),
                    (Cow::from("bar"), 7..10),
                    (Cow::from(""), 11..11),
                ].into(),
                destination: (LinkDestination::Regular("/foo-bar".into()), 17..25),
                title: None,
            },
            LinkReferenceDefinition {
                label: vec![
                    (Cow::from(""), 27..27),
                    (Cow::from("oof"), 32..35),
                    (Cow::from("rab"), 40..43),
                    (Cow::from(""), 44..44),
                ].into(),
                destination: (LinkDestination::Regular("/oof-rab".into()), 47..55),
                title: Some(LinkTitle {
                    kind: TitleMarker::DoubleQuote,
                    value: (Cow::from("this is a title"), 57..72).into()
                }),
            }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn with_multi_line_title() {
        let definition = r#"[fizz-buzz]: <fizz-buzz>
"
this
is a
 multi-line
title
"
"#;

        let result = parse_link_reference_definitions(definition, 0);
        #[rustfmt::skip]
        let expected = vec![
            LinkReferenceDefinition {
                label: ("fizz-buzz", 1..10).into(),
                destination: (LinkDestination::Bracketed("fizz-buzz".into()), 13..23),
                title: Some(LinkTitle {
                    kind: TitleMarker::DoubleQuote,
                    value: vec![
                        (Cow::from(""), 26..26),
                        (Cow::from("this"), 27..31),
                        (Cow::from("is a"), 32..36),
                        (Cow::from("multi-line"), 38..48),
                        (Cow::from("title"), 49..54),
                        (Cow::from(""), 55..55),
                    ].into()
                }),
            }
        ];
        assert_eq!(result, expected);
    }
}
