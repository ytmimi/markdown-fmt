use std::borrow::Cow;
use std::fmt::Write;
use std::iter::Peekable;
use std::ops::Range;
use std::str::FromStr;

use itertools::Itertools;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel};
use pulldown_cmark::{LinkDef, LinkType, Options, Parser, Tag};

use crate::adapters::LooseListExt;
use crate::builder::CodeBlockFormatter;
use crate::config::Config;
use crate::links;
use crate::list::ListMarker;
use crate::paragraph::Paragraph;
use crate::table::TableState;

/// Used to format Markdown inputs.
///
/// To get a [MarkdownFormatter] use [FormatterBuilder::build]
///
/// [FormatterBuilder::build]: crate::FormatterBuilder::build
pub struct MarkdownFormatter {
    code_block_formatter: CodeBlockFormatter,
    config: Config,
}

impl MarkdownFormatter {
    /// Format Markdown input
    ///
    /// ```rust
    /// # use markdown_fmt::FormatterBuilder;
    /// let builder = FormatterBuilder::default();
    /// let formatter = builder.build();
    /// let input = "   #  Header! ";
    /// let rewrite = formatter.format(input).unwrap();
    /// assert_eq!(rewrite, String::from("# Header!"));
    /// ```
    pub fn format(self, input: &str) -> Result<String, std::fmt::Error> {
        // callback that will always revcover broken links
        let mut callback = |broken_link| {
            tracing::trace!("found boken link: {broken_link:?}");
            Some(("".into(), "".into()))
        };

        let mut options = Options::all();
        options.remove(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_with_broken_link_callback(input, options, Some(&mut callback));

        let reference_links = parser
            .reference_definitions()
            .iter()
            .sorted_by(|(_, link_a), (_, link_b)| {
                // We want to sort these in descending order based on the ranges
                // This creates a stack of reference links that we can pop off of.
                link_b.span.start.cmp(&link_a.span.start)
            })
            .map(|(link_lable, LinkDef { dest, title, span })| {
                let full_link = &input[span.clone()];
                if let Some((url, title)) =
                    links::recover_escaped_link_destination_and_title(full_link, title.is_some())
                {
                    (link_lable.to_string(), url, title, span.clone())
                } else {
                    // Couldn't recover URL from source, just use what we've been given
                    (
                        link_lable.to_string(),
                        dest.to_string(),
                        title.clone().map(|s| (s.to_string(), '"')),
                        span.clone(),
                    )
                }
            })
            .collect::<Vec<_>>();

        let iter = parser.into_offset_iter().all_loose_lists();

        let fmt_state = FormatState::new(
            input,
            self.config,
            self.code_block_formatter,
            iter,
            reference_links,
        );
        fmt_state.format()
    }

    /// Helper method to easily initiazlie the [MarkdownFormatter].
    ///
    /// This is marked as `pub(crate)` because users are expected to use the [FormatterBuilder]
    /// When creating a [MarkdownFormatter].
    ///
    /// [FormatterBuilder]: crate::FormatterBuilder
    pub(crate) fn new(code_block_formatter: CodeBlockFormatter, config: Config) -> Self {
        Self {
            code_block_formatter,
            config,
        }
    }
}

type ReferenceLinkDefinition = (String, String, Option<(String, char)>, Range<usize>);

pub(crate) struct FormatState<'i, F, I>
where
    I: Iterator,
{
    /// Raw markdown input
    input: &'i str,
    pub(crate) last_was_softbreak: bool,
    /// Iterator Supplying Markdown Events
    events: Peekable<I>,
    rewrite_buffer: String,
    /// Stores code that we might try to format
    code_block_buffer: String,
    /// Stack that keeps track of nested list markers.
    /// Unordered list markers are one of `*`, `+`, or `-`,
    /// while ordered lists markers start with 0-9 digits followed by a `.` or `)`.
    // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
    // list_markers: Vec<ListMarker>,
    /// Stack that keeps track of indentation.
    indentation: Vec<Cow<'static, str>>,
    /// Stack that keeps track of whether we're formatting inside of another element.
    nested_context: Vec<Tag<'i>>,
    /// A set of reference link definitions that will be output after formatting.
    /// Reference style links contain 3 parts:
    /// 1. Text to display
    /// 2. URL
    /// 3. (Optional) Title
    /// ```markdown
    /// [title]: link "optional title"
    /// ```
    reference_links: Vec<ReferenceLinkDefinition>,
    /// keep track of the current setext header.
    /// ```markdown
    /// Header
    /// ======
    /// ```
    setext_header: Option<&'i str>,
    /// next Start event should push indentation
    needs_indent: bool,
    table_state: Option<TableState<'i>>,
    last_position: usize,
    code_block_formatter: F,
    trim_link_or_image_start: bool,
    /// Handles paragraph formatting.
    paragraph: Option<Paragraph>,
    /// Format configurations
    #[allow(dead_code)]
    config: Config,
}

/// Depnding on the formatting context there are a few different buffers where we might want to
/// write formatted markdown events. The Write impl helps us centralize this logic.
impl<'i, F, I> Write for FormatState<'i, F, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        if let Some(writer) = self.current_buffer() {
            writer.write_str(text)?
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        if let Some(writer) = self.current_buffer() {
            writer.write_fmt(args)?
        }
        Ok(())
    }
}

impl<'i, F, I> FormatState<'i, F, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    /// Peek at the next Markdown Event
    fn peek(&mut self) -> Option<&Event<'i>> {
        self.events.peek().map(|(e, _)| e)
    }

    /// Peek at the next Markdown Event and it's original position in the input
    fn peek_with_range(&mut self) -> Option<(&Event, &Range<usize>)> {
        self.events.peek().map(|(e, r)| (e, r))
    }

    /// Check if the next Event is an `Event::End`
    fn is_next_end_event(&mut self) -> bool {
        matches!(self.peek(), Some(Event::End(_)))
    }

    /// Check if we should write newlines and indentation before the next Start Event
    fn check_needs_indent(&mut self, event: &Event<'i>) {
        self.needs_indent = match self.peek() {
            Some(Event::Start(_) | Event::Rule | Event::Html(_) | Event::End(Tag::Item)) => true,
            Some(Event::End(Tag::BlockQuote)) => matches!(event, Event::End(_)),
            Some(Event::Text(_)) => matches!(event, Event::End(_) | Event::Start(Tag::Item)),
            _ => matches!(event, Event::Rule),
        };
    }

    /// Check if we're formatting a fenced code block
    fn in_fenced_code_block(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::CodeBlock(CodeBlockKind::Fenced(_)))
        )
    }

    /// Check if we're formatting an indented code block
    fn in_indented_code_block(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::CodeBlock(CodeBlockKind::Indented))
        )
    }

    // check if we're formatting a table header
    fn in_table_header(&self) -> bool {
        self.nested_context
            .iter()
            .rfind(|tag| **tag == Tag::TableHead)
            .is_some()
    }

    // check if we're formatting a table row
    fn in_table_row(&self) -> bool {
        self.nested_context
            .iter()
            .rfind(|tag| **tag == Tag::TableRow)
            .is_some()
    }

    /// Check if we're formatting a link
    fn in_link_or_image(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::Link(..) | Tag::Image(..))
        )
    }

    /// Check if we're in a "paragraph". A `Paragraph` might not necessarily be on the
    /// nested_context stack.
    fn in_paragraph(&self) -> bool {
        self.paragraph.is_some()
    }

    /// Check if we're formatting in a nested context
    fn is_nested(&self) -> bool {
        !self.nested_context.is_empty()
    }

    /// Get the length of the indentation
    fn indentation_len(&self) -> usize {
        self.indentation.iter().map(|i| i.len()).sum()
    }

    /// Get an exclusive reference to the current buffer we're writing to. That could be the main
    /// rewrite buffer, the code block buffer, the internal table state, or anything else we're
    /// writing to while reformatting
    fn current_buffer(&mut self) -> Option<&mut dyn std::fmt::Write> {
        if self.in_fenced_code_block() || self.in_indented_code_block() {
            Some(&mut self.code_block_buffer)
        } else if self.in_table_header() || self.in_table_row() {
            self.table_state
                .as_mut()
                .map(|s| s as &mut dyn std::fmt::Write)
        } else if self.in_paragraph() {
            self.paragraph
                .as_mut()
                .map(|p| p as &mut dyn std::fmt::Write)
        } else {
            Some(&mut self.rewrite_buffer)
        }
    }

    /// Check if the current buffer we're writting to is empty
    fn is_current_buffer_empty(&self) -> bool {
        if self.in_fenced_code_block() || self.in_indented_code_block() {
            self.code_block_buffer.is_empty()
        } else if self.in_table_header() || self.in_table_row() {
            self.table_state.as_ref().is_some_and(|s| s.is_empty())
        } else if self.in_paragraph() {
            self.paragraph.as_ref().is_some_and(|p| p.is_empty())
        } else {
            self.rewrite_buffer.is_empty()
        }
    }

    fn count_newlines(&self, range: &Range<usize>) -> usize {
        if self.last_position == range.start {
            return 0;
        }

        let snippet = if self.last_position < range.start {
            // between two markdown evernts
            &self.input[self.last_position..range.start]
        } else {
            // likely in some nested context
            self.input[self.last_position..range.end].trim_end_matches('\n')
        };

        snippet.bytes().filter(|b| *b == b'\n').count()
    }

    fn write_indentation(&mut self, trim_trailing_whiltespace: bool) -> std::fmt::Result {
        let last_non_complete_whitespace_indent = self
            .indentation
            .iter()
            .rposition(|indent| !indent.chars().all(char::is_whitespace));

        let position = if trim_trailing_whiltespace {
            let Some(position) = last_non_complete_whitespace_indent else {
                // All indents are just whitespace. We don't want to push blank lines
                return Ok(());
            };
            position
        } else {
            self.indentation.len()
        };

        // Temporarily take indentation to work around the borrow checker
        let indentation = std::mem::take(&mut self.indentation);

        for (i, indent) in indentation.iter().take(position + 1).enumerate() {
            let is_last = i == position;

            if is_last && trim_trailing_whiltespace {
                self.write_str(indent.trim())?;
            } else {
                self.write_str(indent)?;
            }
        }
        // Put the indentation back!
        self.indentation = indentation;
        Ok(())
    }

    fn write_newlines(&mut self, max_newlines: usize) -> std::fmt::Result {
        self.write_newlines_inner(max_newlines, false)
    }

    fn write_newlines_no_trailing_whitespace(&mut self, max_newlines: usize) -> std::fmt::Result {
        self.write_newlines_inner(max_newlines, true)
    }

    fn write_newlines_inner(
        &mut self,
        max_newlines: usize,
        always_trim_trailing_whitespace: bool,
    ) -> std::fmt::Result {
        if self.is_current_buffer_empty() {
            return Ok(());
        }
        let newlines = self
            .rewrite_buffer
            .chars()
            .rev()
            .take_while(|c| *c == '\n')
            .count();

        let nested = self.is_nested();
        let newlines_to_write = max_newlines.saturating_sub(newlines);
        let next_is_end_event = self.is_next_end_event();

        for i in 0..newlines_to_write {
            let is_last = i == newlines_to_write - 1;

            writeln!(self)?;

            if nested {
                self.write_indentation(!is_last || always_trim_trailing_whitespace)?;
            }
        }
        if !nested {
            self.write_indentation(next_is_end_event || always_trim_trailing_whitespace)?;
        }
        Ok(())
    }

    fn write_reference_link_definition_inner(
        &mut self,
        label: &str,
        dest: &str,
        title: Option<&(String, char)>,
    ) -> std::fmt::Result {
        // empty links can be specified with <>
        let dest = links::format_link_url(dest, true);
        self.write_newlines(1)?;
        if let Some((title, quote)) = title {
            write!(self, r#"[{}]: {dest} {quote}{title}{quote}"#, label.trim())?;
        } else {
            write!(self, "[{}]: {dest}", label.trim())?;
        }
        Ok(())
    }

    fn rewrite_reference_link_definitions(&mut self, range: &Range<usize>) -> std::fmt::Result {
        if self.reference_links.is_empty() {
            return Ok(());
        }
        // use std::mem::take to work around the borrow checker
        let mut reference_links = std::mem::take(&mut self.reference_links);

        loop {
            match reference_links.last() {
                Some((_, _, _, link_range)) if link_range.start > range.start => {
                    // The reference link on the top of the stack comes further along in the file
                    break;
                }
                None => break,
                _ => {}
            }

            let (label, dest, title, link_range) = reference_links.pop().expect("we have a value");
            let newlines = self.count_newlines(&link_range);
            self.write_newlines(newlines)?;
            self.write_reference_link_definition_inner(&label, &dest, title.as_ref())?;
            self.last_position = link_range.end;
            self.needs_indent = true;
        }

        // put the reference_links back
        self.reference_links = reference_links;
        Ok(())
    }

    /// Write out reference links at the end of the file
    fn rewrite_final_reference_links(mut self) -> Result<String, std::fmt::Error> {
        // use std::mem::take to work around the borrow checker
        let reference_links = std::mem::take(&mut self.reference_links);

        // need to iterate in reverse because reference_links is a stack
        for (label, dest, title, range) in reference_links.into_iter().rev() {
            let newlines = self.count_newlines(&range);
            self.write_newlines(newlines)?;

            // empty links can be specified with <>
            self.write_reference_link_definition_inner(&label, &dest, title.as_ref())?;
            self.last_position = range.end
        }
        Ok(self.rewrite_buffer)
    }

    fn join_with_indentation(
        &mut self,
        buffer: &str,
        start_with_indentation: bool,
    ) -> std::fmt::Result {
        let mut lines = buffer.trim_end().lines().peekable();
        while let Some(line) = lines.next() {
            let is_last = lines.peek().is_none();
            let is_next_empty = lines
                .peek()
                .map(|l| l.trim().is_empty())
                .unwrap_or_default();

            if start_with_indentation {
                self.write_indentation(line.trim().is_empty())?;
            }

            if !line.trim().is_empty() {
                self.write_str(line)?;
            }

            if !is_last {
                writeln!(self)?;
            }

            if !is_last && !start_with_indentation {
                self.write_indentation(is_next_empty)?;
            }
        }
        Ok(())
    }
}

impl<'i, F, I> FormatState<'i, F, I>
where
    F: Fn(&str, String) -> String,
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    pub(crate) fn new(
        input: &'i str,
        config: Config,
        code_block_formatter: F,
        iter: I,
        reference_links: Vec<ReferenceLinkDefinition>,
    ) -> Self {
        Self {
            input,
            last_was_softbreak: false,
            events: iter.peekable(),
            rewrite_buffer: String::with_capacity(input.len() * 2),
            code_block_buffer: String::with_capacity(256),
            // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
            // list_markers: vec![],
            indentation: vec![],
            nested_context: vec![],
            reference_links,
            setext_header: None,
            needs_indent: false,
            table_state: None,
            last_position: 0,
            code_block_formatter,
            trim_link_or_image_start: false,
            paragraph: None,
            config,
        }
    }

    fn format_code_buffer(&mut self, info_string: Option<&str>) -> String {
        // use std::mem::take to work around the borrow checker
        let code_block_buffer = std::mem::take(&mut self.code_block_buffer);

        let Some(info_string) = info_string else {
            // An indented code block won't have an info_string
            return code_block_buffer;
        };

        // Call the code_block_formatter fn
        (self.code_block_formatter)(info_string, code_block_buffer)
    }

    fn write_code_block_buffer(&mut self, info_string: Option<&str>) -> std::fmt::Result {
        let code = self.format_code_buffer(info_string);

        if code.trim().is_empty() && info_string.is_some() {
            // The code fence is empty, and a newline should already ahve been added
            // when pushing the opening code fence, so just return.
            return Ok(());
        }

        self.join_with_indentation(&code, info_string.is_some())?;

        if info_string.is_some() {
            // In preparation for the closing code fence write a newline.
            writeln!(self)?
        }

        Ok(())
    }

    /// The main entry point for markdown formatting.
    pub fn format(mut self) -> Result<String, std::fmt::Error> {
        while let Some((event, range)) = self.events.next() {
            tracing::debug!(?event, ?range);
            let mut last_position = self.input[..range.end]
                .bytes()
                .rposition(|b| !b.is_ascii_whitespace())
                .unwrap_or(0);

            match event {
                Event::Start(tag) => {
                    self.rewrite_reference_link_definitions(&range)?;
                    last_position = range.start;
                    self.start_tag(tag.clone(), range)?;
                }
                Event::End(ref tag) => {
                    self.end_tag(tag.clone(), range)?;
                    self.check_needs_indent(&event);
                }
                Event::Text(ref parsed_text) => {
                    last_position = range.end;
                    let starts_with_escape = self.input[..range.start].ends_with('\\');
                    let newlines = self.count_newlines(&range);
                    let text_from_source = &self.input[range];
                    let mut text = if text_from_source.is_empty() {
                        // This seems to happen when the parsed text is whitespace only.
                        // To preserve leading whitespace use the parsed text instead.
                        parsed_text.as_ref()
                    } else {
                        text_from_source
                    };

                    if self.in_link_or_image() && self.trim_link_or_image_start {
                        // Trim leading whitespace from reference links or images
                        text = text.trim_start();
                        // Make sure we only trim leading whitespace once
                        self.trim_link_or_image_start = false
                    }

                    if matches!(
                        self.peek(),
                        Some(Event::End(Tag::Link(..) | Tag::Image(..)))
                    ) {
                        text = text.trim_end();
                    }

                    if self.needs_indent {
                        self.write_newlines(newlines)?;
                        self.needs_indent = false;
                    }

                    if starts_with_escape || self.needs_escape(text) {
                        // recover escape characters
                        write!(self, "\\{text}")?;
                    } else {
                        write!(self, "{text}")?;
                    }
                    self.check_needs_indent(&event);
                }
                Event::Code(_) => {
                    write!(self, "{}", &self.input[range])?;
                }
                Event::SoftBreak => {
                    last_position = range.end;
                    if self.in_link_or_image() {
                        let next_is_end = matches!(
                            self.peek(),
                            Some(Event::End(Tag::Link(..) | Tag::Image(..)))
                        );
                        if self.trim_link_or_image_start || next_is_end {
                            self.trim_link_or_image_start = false
                        } else {
                            write!(self, " ")?;
                        }
                    } else {
                        write!(self, "{}", &self.input[range])?;

                        // paraphraphs write their indentation after reformatting the text
                        if !self.in_paragraph() {
                            self.write_indentation(false)?;
                        }

                        self.last_was_softbreak = true;
                    }
                }
                Event::HardBreak => {
                    write!(self, "{}", &self.input[range])?;
                }
                Event::Html(_) => {
                    let newlines = self.count_newlines(&range);
                    if self.needs_indent {
                        self.write_newlines(newlines)?;
                    }
                    write!(self, "{}", &self.input[range].trim_end_matches('\n'))?;
                    self.check_needs_indent(&event);
                }
                Event::Rule => {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    write!(self, "{}", &self.input[range])?;
                    self.check_needs_indent(&event)
                }
                Event::FootnoteReference(text) => {
                    write!(self, "[^{text}]")?;
                }
                Event::TaskListMarker(done) => {
                    if done {
                        write!(self, "[x]")?;
                    } else {
                        write!(self, "[ ]")?;
                    }
                }
            }
            self.last_position = last_position
        }
        debug_assert!(self.nested_context.is_empty());
        let trailing_newline = self.input.ends_with('\n');
        self.rewrite_final_reference_links().map(|mut output| {
            if trailing_newline {
                output.push('\n');
            }
            output
        })
    }

    fn start_tag(&mut self, tag: Tag<'i>, range: Range<usize>) -> std::fmt::Result {
        match tag {
            Tag::Paragraph => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                self.nested_context.push(tag);
                let capacity = (range.end - range.start) * 2;
                let width = self
                    .config
                    .max_width()
                    .map(|w| w.saturating_sub(self.indentation_len()));
                self.paragraph = Some(Paragraph::new(width, capacity));
            }
            Tag::Heading(level, _, _) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                let full_header = self.input[range].trim();

                if full_header.contains('\n') && full_header.ends_with(['=', '-']) {
                    // support for alternative syntax for H1 and H2
                    // <https://www.markdownguide.org/basic-syntax/#alternate-syntax>
                    let header_marker = full_header.split('\n').last().unwrap().trim();
                    self.setext_header.replace(header_marker);
                    // setext header are handled in `end_tag`
                    return Ok(());
                }

                let header = match level {
                    HeadingLevel::H1 => "# ",
                    HeadingLevel::H2 => "## ",
                    HeadingLevel::H3 => "### ",
                    HeadingLevel::H4 => "#### ",
                    HeadingLevel::H5 => "##### ",
                    HeadingLevel::H6 => "###### ",
                };

                let empty_header = full_header
                    .trim_start_matches(header)
                    .trim_end_matches(|c: char| c.is_whitespace() || matches!(c, '#' | '\\'))
                    .is_empty();

                if empty_header {
                    write!(self, "{}", header.trim())?;
                } else {
                    write!(self, "{header}")?;
                }
            }
            Tag::BlockQuote => {
                // Just in case we're starting a new block quote in a nested context where
                // We alternate indentation levels we want to remove trailing whitespace
                // from the blockquote that we're about to push on top of
                if let Some(indent) = self.indentation.last_mut() {
                    if indent == "> " {
                        *indent = ">".into()
                    }
                }

                let newlines = self.count_newlines(&range);
                if self.needs_indent {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                self.nested_context.push(tag);

                match self.peek_with_range().map(|(e, r)| (e.clone(), r.clone())) {
                    Some((Event::End(Tag::BlockQuote), _)) => {
                        // The next event is `End(BlockQuote)` so the current blockquote is empty!
                        write!(self, ">")?;
                        self.indentation.push(">".into());

                        let snippet = &self.input[range].trim_end();
                        let newlines = snippet.bytes().filter(|b| matches!(b, b'\n')).count();
                        self.write_newlines(newlines)?;
                    }
                    Some((Event::Start(Tag::BlockQuote), next_range)) => {
                        // The next event is `Start(BlockQuote) so we're adding another level
                        // of indentation.
                        write!(self, ">")?;
                        self.indentation.push(">".into());

                        // Now add any missing newlines for empty block quotes between
                        // the current start and the next start
                        let snippet = &self.input[range.start..next_range.start];
                        let newlines = snippet.bytes().filter(|b| matches!(b, b'\n')).count();
                        self.write_newlines(newlines)?;
                    }
                    Some((_, next_range)) => {
                        // Now add any missing newlines for empty block quotes between
                        // the current start and the next start
                        let snippet = &self.input[range.start..next_range.start];
                        let newlines = snippet.bytes().filter(|b| matches!(b, b'\n')).count();

                        self.indentation.push("> ".into());
                        if newlines > 0 {
                            write!(self, ">")?;
                            self.write_newlines(newlines)?;
                        } else {
                            write!(self, "> ")?;
                        }
                    }
                    None => {
                        // Peeking at the next event should always return `Some()` for start events
                        unreachable!("At the very least we'd expect an `End(BlockQuote)` event.");
                    }
                }
            }
            Tag::CodeBlock(ref kind) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                match kind {
                    CodeBlockKind::Fenced(info_string) => {
                        rewrite_marker(self.input, &range, self)?;

                        if info_string.is_empty() {
                            writeln!(self)?;
                            self.nested_context.push(tag);
                            return Ok(());
                        }

                        let starts_with_space = self.input[range.clone()]
                            .trim_start_matches(['`', '~'])
                            .starts_with(char::is_whitespace);

                        let info_string = self.input[range]
                            .lines()
                            .next()
                            .unwrap_or_else(|| info_string.as_ref())
                            .trim_start_matches(['`', '~'])
                            .trim();

                        if starts_with_space {
                            writeln!(self, " {info_string}")?;
                        } else {
                            writeln!(self, "{info_string}")?;
                        }
                    }
                    CodeBlockKind::Indented => {
                        // TODO(ytmimi) support tab as an indent
                        let indentation = "    ";

                        if !matches!(
                            self.peek(),
                            Some(Event::End(Tag::CodeBlock(CodeBlockKind::Indented)))
                        ) {
                            // Only write indentation if this isn't an empty indented code block
                            self.write_str(indentation)?;
                        }

                        self.indentation.push(indentation.into());
                    }
                }
                self.nested_context.push(tag);
            }
            Tag::List(_) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
                // let list_marker = ListMarker::from_str(&self.input[range])
                //    .expect("Should be able to parse a list marker");
                // self.list_markers.push(list_marker);
                self.nested_context.push(tag);
            }
            Tag::Item => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                }

                let empty_list_item = match self.events.peek() {
                    Some((Event::End(Tag::Item), _)) => true,
                    Some((_, next_range)) => {
                        let snippet = &self.input[range.start..next_range.start];
                        // It's an empty list if there are newlines between the list marker
                        // and the next event. For example,
                        //
                        // ```markdown
                        // -
                        //   foo
                        // ```
                        snippet.bytes().filter(|b| matches!(b, b'\n')).count() > 0
                    }
                    None => false,
                };

                // We need to push a newline and indentation before the next event if
                // this is an empty list item
                self.needs_indent = empty_list_item;

                let list_marker = ListMarker::from_str(&self.input[range])
                    .expect("Should be able to parse a list marker");
                // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
                // Take list_marker so we can use `write!(self, ...)`
                // let mut list_marker = self
                //     .list_markers
                //     .pop()
                //     .expect("can't have list item without marker");
                let marker_char = list_marker.marker_char();
                match &list_marker {
                    ListMarker::Ordered { number, .. } if empty_list_item => {
                        let zero_padding = list_marker.zero_padding();
                        write!(self, "{zero_padding}{number}{marker_char}")?;
                    }
                    ListMarker::Ordered { number, .. } => {
                        let zero_padding = list_marker.zero_padding();
                        write!(self, "{zero_padding}{number}{marker_char} ")?;
                    }
                    ListMarker::Unordered(_) if empty_list_item => {
                        write!(self, "{marker_char}")?;
                    }
                    ListMarker::Unordered(_) => {
                        write!(self, "{marker_char} ")?;
                    }
                }

                self.nested_context.push(tag);
                // Increment the list marker in case this is a ordered list and
                // swap the list marker we took earlier
                self.indentation.push(list_marker.indentation());
                // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
                // list_marker.increment_count();
                // self.list_markers.push(list_marker)
            }
            Tag::FootnoteDefinition(label) => {
                write!(self, "[^{label}]: ")?;
            }
            Tag::Emphasis => {
                rewrite_marker_with_limit(self.input, &range, self, Some(1))?;
            }
            Tag::Strong => {
                rewrite_marker_with_limit(self.input, &range, self, Some(2))?;
            }
            Tag::Strikethrough => {
                rewrite_marker(self.input, &range, self)?;
            }
            Tag::Link(link_type, ..) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                let email_or_auto = matches!(link_type, LinkType::Email | LinkType::Autolink);
                let opener = if email_or_auto { "<" } else { "[" };
                self.write_str(opener)?;
                self.nested_context.push(tag);

                if matches!(self.peek(), Some(Event::Text(_) | Event::SoftBreak)) {
                    self.trim_link_or_image_start = true
                }
            }
            Tag::Image(..) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                write!(self, "![")?;
                self.nested_context.push(tag);

                if matches!(self.peek(), Some(Event::Text(_) | Event::SoftBreak)) {
                    self.trim_link_or_image_start = true
                }
            }
            Tag::Table(ref alignment) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                self.table_state.replace(TableState::new(alignment.clone()));
                write!(self, "|")?;
                self.indentation.push("|".into());
                self.nested_context.push(tag);
            }
            Tag::TableHead => {
                self.nested_context.push(tag);
            }
            Tag::TableRow => {
                self.nested_context.push(tag);
                if let Some(state) = self.table_state.as_mut() {
                    state.push_row()
                }
            }
            Tag::TableCell => {
                if !matches!(self.peek(), Some(Event::End(Tag::TableCell))) {
                    return Ok(());
                }

                if let Some(state) = self.table_state.as_mut() {
                    state.write(String::new().into());
                }
            }
        }
        Ok(())
    }

    fn end_tag(&mut self, tag: Tag<'i>, range: Range<usize>) -> std::fmt::Result {
        match tag {
            Tag::Paragraph => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));

                if let Some(p) = self.paragraph.take() {
                    self.join_with_indentation(&p.into_buffer(), false)?;
                }
            }
            Tag::Heading(_, fragment_identifier, classes) => {
                match (fragment_identifier, classes.is_empty()) {
                    (Some(id), false) => {
                        let classes = rewirte_header_classes(classes)?;
                        write!(self, " {{#{id}{classes}}}")?;
                    }
                    (Some(id), true) => {
                        write!(self, " {{#{id}}}")?;
                    }
                    (None, false) => {
                        let classes = rewirte_header_classes(classes)?;
                        write!(self, " {{{}}}", classes.trim())?;
                    }
                    (None, true) => {}
                }

                if let Some(marker) = self.setext_header.take() {
                    self.write_newlines(1)?;
                    write!(self, "{marker}")?;
                }
            }
            Tag::BlockQuote => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    // Recover empty block quote lines
                    if let Some(last) = self.indentation.last_mut() {
                        // Avoid trailing whitespace by replacing the last indentation with '>'
                        *last = ">".into()
                    }
                    self.write_newlines(newlines)?;
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));

                let popped_indentation = self
                    .indentation
                    .pop()
                    .expect("we pushed a blockquote marker in start_tag");
                if let Some(indentation) = self.indentation.last_mut() {
                    if indentation == ">" {
                        *indentation = popped_indentation
                    }
                }
            }
            Tag::CodeBlock(ref kind) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref(), Some(&tag));

                match kind {
                    CodeBlockKind::Fenced(info_string) => {
                        self.write_code_block_buffer(Some(info_string))?;
                        // write closing code fence
                        self.write_indentation(false)?;
                        rewrite_marker(self.input, &range, self)?;
                    }
                    CodeBlockKind::Indented => {
                        // Maybe we'll consider formatting indented code blocks??
                        self.write_code_block_buffer(None)?;

                        let popped_indentation = self
                            .indentation
                            .pop()
                            .expect("we added 4 spaces in start_tag");
                        debug_assert_eq!(popped_indentation, "    ");
                    }
                }
            }
            Tag::List(_) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
                // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
                // self.list_markers.pop();

                // To prevent the next code block from being interpreted as a list we'll add an
                // HTML comment See https://spec.commonmark.org/0.30/#example-308, which states:
                //
                //     To separate consecutive lists of the same type, or to separate a list from an
                //     indented code block that would otherwise be parsed as a subparagraph of the
                //     final list item, you can insert a blank HTML comment
                if let Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Indented))) = self.peek() {
                    self.write_newlines(1)?;
                    writeln!(self, "<!-- Don't absorb code block into list -->")?;
                    write!(self, "<!-- Consider a fenced code block instead -->")?;
                };
            }
            Tag::Item => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines_no_trailing_whitespace(newlines)?;
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
                let popped_indentation = self.indentation.pop();
                debug_assert!(popped_indentation.is_some());

                // if the next event is a Start(Item), then we need to set needs_indent
                self.needs_indent = matches!(self.peek(), Some(Event::Start(Tag::Item)));
            }
            Tag::FootnoteDefinition(_label) => {}
            Tag::Emphasis => {
                rewrite_marker_with_limit(self.input, &range, self, Some(1))?;
            }
            Tag::Strong => {
                rewrite_marker_with_limit(self.input, &range, self, Some(2))?;
            }
            Tag::Strikethrough => {
                rewrite_marker(self.input, &range, self)?;
            }
            Tag::Link(ref link_type, ref url, ref title)
            | Tag::Image(ref link_type, ref url, ref title) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref(), Some(&tag));

                let text = &self.input[range.clone()];

                match link_type {
                    LinkType::Inline => {
                        if let Some((source_url, title_and_quote)) =
                            crate::links::find_inline_url_and_title(text)
                        {
                            self.write_inline_link(&source_url, title_and_quote)?;
                        } else {
                            let title = if title.is_empty() {
                                None
                            } else {
                                Some((title, '"'))
                            };
                            self.write_inline_link(url, title)?;
                        }
                    }
                    LinkType::Reference | LinkType::ReferenceUnknown => {
                        let label = crate::links::find_reference_link_label(text);
                        write!(self, "][{label}]")?;
                    }
                    LinkType::Collapsed | LinkType::CollapsedUnknown => write!(self, "][]")?,
                    LinkType::Shortcut | LinkType::ShortcutUnknown => write!(self, "]")?,
                    LinkType::Autolink | LinkType::Email => write!(self, ">")?,
                }
            }
            Tag::Table(_) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
                if let Some(state) = self.table_state.take() {
                    self.join_with_indentation(&state.format()?, false)?;
                }
                let popped_indentation = self.indentation.pop().expect("we added `|` in start_tag");
                debug_assert_eq!(popped_indentation, "|");
            }
            Tag::TableRow | Tag::TableHead => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
            }
            Tag::TableCell => {
                if let Some(state) = self.table_state.as_mut() {
                    // We finished formatting this cell. Setup the state to format the next cell
                    state.increment_col_index()
                }
            }
        }
        Ok(())
    }
}

/// Find some marker that denotes the start of a markdown construct.
/// for example, `**` for bold or `_` for italics.
fn find_marker<'i, P>(input: &'i str, range: &Range<usize>, predicate: P) -> &'i str
where
    P: FnMut(char) -> bool,
{
    let end = if let Some(position) = input[range.start..].chars().position(predicate) {
        range.start + position
    } else {
        range.end
    };
    &input[range.start..end]
}

/// Find some marker, but limit the size
fn rewrite_marker_with_limit<W: std::fmt::Write>(
    input: &str,
    range: &Range<usize>,
    writer: &mut W,
    size_limit: Option<usize>,
) -> std::fmt::Result {
    let marker_char = input[range.start..].chars().next().unwrap();
    let marker = find_marker(input, range, |c| c != marker_char);
    if let Some(mark_max_width) = size_limit {
        write!(writer, "{}", &marker[..mark_max_width])
    } else {
        write!(writer, "{marker}")
    }
}

/// Finds a marker in the source text and writes it to the buffer
fn rewrite_marker<W: std::fmt::Write>(
    input: &str,
    range: &Range<usize>,
    writer: &mut W,
) -> std::fmt::Result {
    rewrite_marker_with_limit(input, range, writer, None)
}

/// Rewrite a list of h1, h2, h3, h4, h5, h6 classes
fn rewirte_header_classes(classes: Vec<&str>) -> Result<String, std::fmt::Error> {
    let item_len = classes.iter().map(|i| i.len()).sum::<usize>();
    let capacity = item_len + classes.len() * 2;
    let mut result = String::with_capacity(capacity);
    for class in classes {
        write!(result, " .{class}")?;
    }
    Ok(result)
}
