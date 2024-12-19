use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::Write;
use std::iter::Peekable;
use std::ops::Range;
use std::str::FromStr;

use itertools::Itertools;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, TagEnd};
use pulldown_cmark::{LinkType, Parser, Tag};

use crate::adapters::{ListEndAtLastItemExt, LooseListExt};
use crate::builder::{CodeBlockContext, CodeBlockFormatter};
use crate::config::Config;
use crate::definition_list::DefinitionListTitle;
use crate::escape::needs_escape;
use crate::footnote::FootnoteDefinition;
use crate::header::{Header, HeaderKind};
use crate::html::starts_with_html_block_identifier;
use crate::links::{LinkReferenceDefinition, LinkWriter, parse_link_reference_definitions};
use crate::list::{LIST_START_CHARS, ListMarker};
use crate::paragraph::Paragraph;
use crate::table::TableState;
use crate::utils::{
    count_newlines, count_trailing_spaces, get_spaces, sequence_ends_on_escape, split_lines,
};
use crate::writer::{
    MarkdownContext, MarkdownWriter, WriteContext, write_context, writeln_context,
};

static DEFINITION_LIST_INDENTATION: &str = "  ";

// Defined using a macro so that the parsing options can be shared with tests for consistency.
#[doc(hidden)]
#[macro_export]
macro_rules! pulldown_cmark_options {
    () => {
        pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_FOOTNOTES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS
            | pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES
            | pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_DEFINITION_LIST
    };
}

/// Used to format Markdown inputs.
///
/// To get a [MarkdownFormatter] use [FormatBuilder::build]
///
/// [FormatBuilder::build]: crate::FormatBuilder::build
pub struct MarkdownFormatter {
    code_block_formatter: CodeBlockFormatter,
    config: RefCell<Config>,
}

impl MarkdownFormatter {
    /// Format Markdown input
    ///
    /// ```rust
    /// # use markdown_fmt::FormatBuilder;
    /// let builder = FormatBuilder::default();
    /// let formatter = builder.build();
    /// let input = "   #  Header! ";
    /// let rewrite = formatter.format(input).unwrap();
    /// assert_eq!(rewrite, String::from("# Header!"));
    /// ```
    pub fn format(&self, input: &str) -> Result<String, std::fmt::Error> {
        // callback that will always revcover broken links
        let mut callback = |broken_link| {
            tracing::trace!("found boken link: {broken_link:?}");
            Some(("".into(), "".into()))
        };

        let options = pulldown_cmark_options!();

        let parser = Parser::new_with_broken_link_callback(input, options, Some(&mut callback));
        let iter = parser
            .into_offset_iter()
            .all_loose_lists()
            .list_end_at_last_item();

        let fmt_state = FormatState::new(input, self, iter);
        fmt_state.format()
    }

    pub(crate) fn get_config<F, O>(&self, f: F) -> O
    where
        F: Fn(&Config) -> O,
    {
        f(&self.config.borrow())
    }

    pub(crate) fn set_config<F>(&self, f: F)
    where
        F: Fn(&mut Config),
    {
        f(&mut self.config.borrow_mut())
    }

    /// Helper method to easily initiazlie the [MarkdownFormatter].
    ///
    /// This is marked as `pub(crate)` because users are expected to use the [FormatBuilder]
    /// When creating a [MarkdownFormatter].
    ///
    /// [FormatBuilder]: crate::FormatBuilder
    pub(crate) fn new(code_block_formatter: CodeBlockFormatter, config: Config) -> Self {
        Self {
            code_block_formatter,
            config: RefCell::new(config),
        }
    }
}

pub(crate) struct FormatState<'i, 'm, I>
where
    I: Iterator,
{
    /// Raw markdown input
    input: &'i str,
    pub(crate) last_was_softbreak: bool,
    /// Iterator Supplying Markdown Events
    events: Peekable<I>,
    rewrite_buffer: String,
    /// Stack that keeps track of nested list markers.
    /// Unordered list markers are one of `*`, `+`, or `-`,
    /// while ordered lists markers start with 0-9 digits followed by a `.` or `)`.
    // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
    // list_markers: Vec<ListMarker>,
    /// Stack that keeps track of indentation.
    indentation: Vec<Cow<'static, str>>,
    /// Stack that keeps track of whether we're formatting inside of another element.
    nested_context: Vec<Tag<'i>>,
    /// Stack of writers that handle individual Markdown structures like code blocks, tables,
    /// paragraphs, headers and footnote definitions
    writers: Vec<MarkdownWriter<'i>>,
    /// A set of reference link definitions that will be output after formatting.
    /// Reference style links contain 3 parts:
    /// 1. Text to display
    /// 2. URL
    /// 3. (Optional) Title
    /// ```markdown
    /// [title]: link "optional title"
    /// ```
    #[allow(dead_code)]
    // TODO(ytmimi) will come in handy when adding an option to defer
    // rewriting link reference definitions until the end of the document
    reference_links: Vec<LinkReferenceDefinition<'i>>,
    /// next Start event should push indentation
    needs_indent: bool,
    last_position: usize,
    // Last event emitted from the inner iterator
    last_event: Option<I::Item>,
    formatter: &'m MarkdownFormatter,
    /// For some reason the indentation of indented code blocks is dependeant on the relative
    /// position of the first piece of content within the definition list, and not the starting
    /// position of the `:`.
    empty_definition_list_definition_marker: bool,
}

/// Depnding on the formatting context there are a few different buffers where we might want to
/// write formatted markdown events. The Write impl helps us centralize this logic.
impl<'i, I> Write for FormatState<'i, '_, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let writer = self.current_buffer();
        writer.write_str(text)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        let writer = self.current_buffer();
        writer.write_fmt(args)
    }
}

/// Pass along context about the current [Event] that's being rewritten
impl<'i, I> WriteContext<'i> for FormatState<'i, '_, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    fn write_context_str(&mut self, ctx: MarkdownContext<'_, 'i>, s: &str) -> std::fmt::Result {
        let writer = self.current_buffer();
        writer.write_context_str(ctx, s)
    }
}

impl<'i, I> FormatState<'i, '_, I>
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
            Some(Event::Start(_) | Event::Rule | Event::Html(_) | Event::End(TagEnd::Item)) => true,
            Some(Event::End(TagEnd::BlockQuote(..))) => matches!(event, Event::End(_)),
            Some(Event::Text(_)) => matches!(event, Event::End(_) | Event::Start(Tag::Item)),
            _ => matches!(event, Event::Rule),
        };
    }

    /// check if we're in a blockquote
    pub(crate) fn in_blockquote(&self) -> bool {
        self.nested_context
            .iter()
            .any(|t| matches!(t, Tag::BlockQuote(_)))
    }

    /// Check if we're formatting a link
    fn in_link_or_image(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::Link { .. } | Tag::Image { .. })
        )
    }

    /// Check if we're currently formatting an HTML block
    fn in_html_block(&self) -> bool {
        matches!(self.nested_context.last(), Some(Tag::HtmlBlock))
    }

    /// Check if we're in a "paragraph". A `Paragraph` might not necessarily be on the
    /// nested_context stack.
    fn in_paragraph(&self) -> bool {
        self.writers
            .last()
            .is_some_and(|w| matches!(w, MarkdownWriter::Paragraph(_)))
    }

    /// Check if we're currently formatting the definition of a definition list
    fn in_definition_list_definition(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::DefinitionListDefinition)
        )
    }

    /// Check if we're currently formatting a definition list title
    fn in_definition_list_title(&self) -> bool {
        self.writers
            .last()
            .is_some_and(|w| matches!(w, MarkdownWriter::DefinitionListTitle(_)))
    }

    /// Check if we're in a Table
    fn in_table(&self) -> bool {
        self.writers
            .last()
            .is_some_and(|w| matches!(w, MarkdownWriter::Table(_)))
    }

    /// Check if we're formatting in a nested context
    fn is_nested(&self) -> bool {
        !self.nested_context.is_empty()
    }

    /// Get the length of the indentation
    fn indentation_len(&self) -> usize {
        self.indentation.iter().map(|i| i.len()).sum()
    }

    /// Dynamically determine how much indentation to use for indented code blocks
    fn indented_code_block_indentation(&self) -> &'static str {
        if self.empty_definition_list_definition_marker && self.in_definition_list_definition() {
            "   "
        } else {
            "    "
        }
    }

    /// Get an exclusive reference to the current buffer we're writing to. That could be the main
    /// rewrite buffer, the code block buffer, the internal table state, or anything else we're
    /// writing to while reformatting
    fn current_buffer(&mut self) -> &mut dyn WriteContext {
        if let Some(writer) = self.writers.last_mut() {
            writer as &mut dyn WriteContext
        } else {
            &mut self.rewrite_buffer
        }
    }

    /// Check if the current buffer we're writting to is empty
    fn is_current_buffer_empty(&self) -> bool {
        if let Some(writer) = self.writers.last() {
            writer.is_empty()
        } else {
            self.rewrite_buffer.is_empty()
        }
    }

    // Does not count trailing newlines. For some reason that caused issues
    fn count_newlines_in_range(&self, range: &Range<usize>) -> usize {
        let snippet = &self.input[range.clone()];
        count_newlines(snippet.trim_end_matches(['\r', '\n']))
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
            self.input[self.last_position..range.end].trim_end_matches(['\r', '\n'])
        };
        count_newlines(snippet)
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
        if max_newlines == 0 {
            return Ok(());
        }
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
            .take_while(|c| *c == '\n' || *c == '\r')
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

    fn rewrite_reference_link_definitions_inner(
        &mut self,
        link_defs: Vec<LinkReferenceDefinition>,
    ) -> std::fmt::Result {
        // TODO(ytmimi) Add an option to defer writing links until the end
        for link_def in link_defs {
            let link_range = link_def.range();
            let newlines = self.count_newlines(&link_range);
            self.write_newlines(newlines)?;
            link_def.write(self)?;
            self.last_position = link_range.end;
            self.needs_indent = true;
        }
        Ok(())
    }

    fn rewrite_reference_link_definitions(&mut self, range: &Range<usize>) -> std::fmt::Result {
        let snippet = &self.input[range.clone()];
        let link_defs = parse_link_reference_definitions(snippet, range.start);

        if link_defs.is_empty() {
            return Ok(());
        }
        self.rewrite_reference_link_definitions_inner(link_defs)
    }

    /// Write out reference links at the end of the file
    fn rewrite_final_reference_links(mut self) -> Result<String, std::fmt::Error> {
        let range = self.last_position..self.input.len();
        self.rewrite_reference_link_definitions(&range)?;
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

    fn trim_leading_indentation<'a>(&self, s: &'a str) -> &'a str {
        let mut output = s.trim_start();
        for indent in self.indentation.iter() {
            if indent.starts_with('>') {
                output = output.strip_prefix('>').unwrap_or(output).trim_start();
            } else {
                output = output.trim_start();
            }
        }
        output
    }

    /// Rewrite a single [`Event`] that spans over multiple lines
    fn rewrite_multiline_event(&mut self, event: &Event<'i>, snippet: &'i str) -> std::fmt::Result {
        // check if the input snippet ends with a newline so we can preserve it.
        let ends_with_newline = snippet.ends_with('\n');

        let mut iter = split_lines(snippet).peekable();

        while let Some(s) = iter.next() {
            let is_last = iter.peek().is_none();

            // We want to trim leading indentation characters like `>`
            // and any non-space whitespace characters.
            let line = self.trim_leading_indentation(s);

            let trailing_space_count = count_trailing_spaces(line);
            let trailing_spaces = get_spaces(trailing_space_count);

            let line = line.trim_end();
            let needs_escape = self.needs_escape(line, true);

            match needs_escape {
                Some(escape_kind) if escape_kind.multi_character_escape() => {
                    let marker = escape_kind.marker();
                    for c in line.chars() {
                        if marker == c {
                            write_context!(self, Escape, "\\{c}")?;
                        } else {
                            self.write_char(c)?;
                        }
                    }
                    self.write_event_str(event, &trailing_spaces)?;
                }
                Some(_) => {
                    write_context!(self, Escape, "\\{line}{trailing_spaces}")?;
                }
                None => {
                    write_context!(self, event, "{line}{trailing_spaces}")?;
                }
            }

            if !is_last || ends_with_newline {
                writeln_context!(self, event)?;
            }
        }
        Ok(())
    }
}

impl<'i, 'm, I> FormatState<'i, 'm, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    pub(crate) fn new(input: &'i str, formatter: &'m MarkdownFormatter, iter: I) -> Self {
        Self {
            input,
            last_was_softbreak: false,
            events: iter.peekable(),
            rewrite_buffer: String::with_capacity(input.len() * 2),
            // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
            // list_markers: vec![],
            indentation: vec![],
            nested_context: vec![],
            reference_links: vec![],
            writers: vec![],
            needs_indent: false,
            last_position: 0,
            last_event: None,
            formatter,
            empty_definition_list_definition_marker: false,
        }
    }

    fn format_code_buffer(
        &mut self,
        info_string: Option<&str>,
        unformatted_code: String,
    ) -> String {
        let Some(info_string) = info_string else {
            // An indented code block won't have an info_string
            return unformatted_code;
        };

        // To prepare for a nested code block that might recursively call into the markdown
        // formatter we should get a snapshot of the width and update it for the nested context.
        let indentation = self.indentation_len();
        let current_max_width = self.formatter.get_config(|c| c.max_width());
        let new_max_with = current_max_width.map(|width| width.saturating_sub(indentation));
        self.formatter.set_config(|c| c.set_max_width(new_max_with));

        let rewrite = if info_string.contains("markdown") {
            // recursively call into the the `MarkdownFormatter`
            self.formatter
                .format(&unformatted_code)
                .unwrap_or(unformatted_code)
        } else {
            let ctx = CodeBlockContext {
                indentation,
                max_width: current_max_width,
            };
            // Call the code_block_formatter fn
            (self.formatter.code_block_formatter)(&ctx, info_string, unformatted_code)
        };

        // restore the width after formatting the code block
        self.formatter
            .set_config(|c| c.set_max_width(current_max_width));

        rewrite
    }

    fn write_code_block_buffer(
        &mut self,
        info_string: Option<&str>,
        unformatted_code: String,
    ) -> std::fmt::Result {
        let code = self.format_code_buffer(info_string, unformatted_code);

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
            tracing::debug!(?event, ?range, last_position = self.last_position);
            let mut last_position = if matches!(event, Event::HardBreak) {
                range.end
            } else {
                self.input[..range.end]
                    .bytes()
                    .rposition(|b| !b.is_ascii_whitespace())
                    .map(
                        |offset| offset + 1, /* +1 to start on the whitespace or end of input */
                    )
                    .unwrap_or(0)
            };
            let last_range = range.clone();
            match event {
                Event::Start(ref tag) => {
                    last_position = range.start;
                    self.start_tag(tag.clone(), range)?;
                    // self.last_position might be modified in `start_tag` if we need to recover
                    // link reference definitions. To prevent resetting it, make sure
                    // it stays at self.last_position
                    if last_position < self.last_position {
                        last_position = self.last_position;
                    }
                }
                Event::End(ref tag) => {
                    self.end_tag(*tag, range.clone())?;
                    self.check_needs_indent(&event);
                }
                Event::Text(ref parsed_text) => {
                    last_position = range.end;
                    let current_input_snippet = &self.input[..range.start];
                    let starts_with_escape = if current_input_snippet.ends_with('\n') {
                        // Can't start with an esacpe if it ends on a newline
                        false
                    } else {
                        //
                        current_input_snippet
                            .lines()
                            .last()
                            .map_or(false, sequence_ends_on_escape)
                    };
                    let newlines = self.count_newlines(&range);
                    let text_from_source = &self.input[range.clone()];
                    let text = if text_from_source.is_empty() || self.in_html_block() {
                        // This seems to happen when the parsed text is whitespace only.
                        // To preserve leading whitespace use the parsed text instead.
                        parsed_text.as_ref()
                    } else {
                        text_from_source
                    };

                    if self.needs_indent {
                        self.write_newlines(newlines)?;
                        self.needs_indent = false;
                    }

                    // aggressively escape if we're in a definition list
                    let needs_escape = self.needs_escape(text, false);

                    let could_be_interpreted_as_html =
                        |t: &str, state: &mut FormatState<'i, 'm, I>| -> bool {
                            if state.last_was_softbreak
                                && t == "<"
                                && matches!(
                                    state.peek(),
                                    Some(Event::Text(t))
                                        if starts_with_html_block_identifier(t)
                                )
                            {
                                return true;
                            }

                            if !state.in_blockquote() {
                                return false;
                            }

                            // IF we're in a blockquote it means that a softbreak could lead to
                            // a scenario where we add a `>`, which causes `<!` + `>` to get
                            // interpreted as an inline html element.
                            let last_was_lt = matches!(
                                &state.last_event, Some((Event::Text(t), _)) if t.ends_with('<')
                            );

                            last_was_lt && t.starts_with('!')
                        };

                    // Prevent the next pass from interpreting this as a hard break
                    let should_escape_an_escape =
                        |t: &str, state: &mut FormatState<'i, 'm, I>, r: Range<usize>| -> bool {
                            let not_empty_paragraph =
                                !state.in_paragraph() && !state.is_current_buffer_empty();
                            if not_empty_paragraph || !t.chars().all(|c| c == '\\') {
                                return false;
                            }

                            // If the next is a soft break then we want to escape this `\\`
                            let Some(Event::SoftBreak) = state.peek() else {
                                return false;
                            };

                            let Some(last_line) = &self.input[..r.end].lines().last() else {
                                return false;
                            };

                            sequence_ends_on_escape(last_line)
                        };

                    match needs_escape {
                        Some(escape_kind)
                            if escape_kind.multi_character_escape() && !starts_with_escape =>
                        {
                            let marker = escape_kind.marker();
                            for c in text.chars() {
                                if marker == c {
                                    write_context!(self, Escape, "\\{c}")?;
                                } else {
                                    self.write_char(c)?;
                                }
                            }
                        }
                        _ => {
                            if starts_with_escape
                                || needs_escape.is_some()
                                || could_be_interpreted_as_html(text, &mut self)
                                || should_escape_an_escape(text, &mut self, range)
                                || (self.in_table() && text.starts_with('|'))
                            {
                                // recover escape characters
                                write_context!(self, Escape, "\\{text}")?;
                            } else {
                                write_context!(self, &event, "{text}")?;
                            }
                        }
                    }
                    self.check_needs_indent(&event);
                }
                Event::Code(_) => {
                    let snippet = &self.input[range.clone()];
                    if count_newlines(snippet) > 0 {
                        let snippet = snippet.trim_matches('`');
                        // write the opening and closing markers separately so they aren't escaped.
                        rewrite_marker(self.input, &range, &mut self)?;
                        self.rewrite_multiline_event(&event, snippet)?;
                        rewrite_marker(self.input, &range, &mut self)?;
                    } else {
                        write!(self, "{}", snippet)?;
                    }
                }
                Event::InlineHtml(_) => {
                    let snippet = &self.input[range.clone()];
                    self.rewrite_multiline_event(&event, snippet)?;
                }
                Event::SoftBreak => {
                    last_position = range.end;

                    writeln_context!(self, &event)?;

                    // paraphraphs write their indentation after reformatting the text
                    if !(self.in_paragraph()
                        || self.in_definition_list_title()
                        || self.in_link_or_image())
                    {
                        self.write_indentation(false)?;
                    }
                }
                Event::HardBreak => {
                    let hard_break = match &self.input[range] {
                        "\\\r" | "\\\r\n" | "\\\n" => "\\\n",
                        "  \r" | "  \r\n" | "  \n" => "  \n",
                        h => h,
                    };

                    self.write_event_str(&event, hard_break)?;
                }
                Event::Html(_) => {
                    let newlines = self.count_newlines(&range);
                    if self.needs_indent {
                        self.write_newlines(newlines)?;
                    }
                    let snippet = &self.input[range].trim_end();
                    self.write_event_str(&event, snippet)?;
                    self.check_needs_indent(&event);
                }
                Event::Rule => {
                    let reference_definition_range = self.last_position..range.start;
                    self.rewrite_reference_link_definitions(&reference_definition_range)?;
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.write_event_str(&event, self.input[range].trim_end())?;
                    self.check_needs_indent(&event)
                }
                Event::FootnoteReference(ref text) => {
                    write_context!(self, &event, "[^{text}]")?;
                }
                Event::TaskListMarker(done) => {
                    if done {
                        write_context!(self, &event, "[x] ")?;
                    } else {
                        write_context!(self, &event, "[ ] ")?;
                    }
                }
                Event::DisplayMath(..) | Event::InlineMath(..) => {
                    unreachable!("pulldown_cmark::Options::ENABLE_MATH is not configured")
                }
            }
            self.last_was_softbreak = matches!(event, Event::SoftBreak);
            self.last_position = last_position;
            self.last_event = Some((event, last_range));
        }
        debug_assert!(self.nested_context.is_empty());

        let trailing_newline = self
            .input
            .rfind(|c: char| !c.is_whitespace())
            .is_some_and(|start| self.input[start..].contains(['\r', '\n']));

        self.rewrite_final_reference_links().map(|mut output| {
            // Prevent extranious newlines at the end of the output
            while output.ends_with(['\r', '\n']) {
                output.pop();
            }

            if trailing_newline {
                output.push('\n');
            }
            output
        })
    }

    fn start_tag(&mut self, tag: Tag<'i>, range: Range<usize>) -> std::fmt::Result {
        // These all come after we're already in the context of a Table.
        // I don't think it's possible for a reference link definition to come before these tags.
        if !matches!(tag, Tag::TableHead | Tag::TableRow | Tag::TableCell) {
            let reference_definition_range = self.last_position..range.start;
            self.rewrite_reference_link_definitions(&reference_definition_range)?;
        }

        match tag {
            Tag::Paragraph => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                let capacity = (range.end - range.start) * 2;
                let (max_width, should_reflow_text) = self.formatter.get_config(|c| {
                    let width = c
                        .max_width()
                        .map(|w| w.saturating_sub(self.indentation_len()));
                    (width, c.reflow_text())
                });
                let paragraph = Paragraph::new(max_width, should_reflow_text, capacity);
                self.writers.push(paragraph.into());
            }
            Tag::Heading { .. } => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                let full_header = self.input[range].trim();
                let header = Header::new(
                    // Take the indentaiton so that we don't accidentally write indentation into the
                    // headers for setext headers that may span multiple lines.
                    // We will restore the indentation after we're done formatting the header.
                    std::mem::take(&mut self.indentation),
                    full_header,
                    tag,
                );
                self.writers.push(header.into())
            }
            // `pulldown_cmark::Options::ENABLE_GFM` is not configured so we shouldn't have
            // a `BlockQuoteKind`` in the `Tag::BlockQuote`
            Tag::BlockQuote(_) => {
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

                // Anchor the last_position at the start of the blockquote. This prevents picking
                // up extra newlines in case we need to recover any refernce link definitions
                self.last_position = range.start;

                self.nested_context.push(tag);

                // FIXME(ytmimi) recovering link-reference-definitions adds some complexity here.
                // Hoping to find a way to simplify this in the future.
                match self.peek_with_range().map(|(e, r)| (e.clone(), r.clone())) {
                    Some((Event::End(TagEnd::BlockQuote(..)), _)) => {
                        let snippet = &self.input[range.clone()];
                        let link_defs = parse_link_reference_definitions(snippet, range.start);
                        if !link_defs.is_empty() {
                            write!(self, "> ")?;
                            self.indentation.push("> ".into());
                            self.rewrite_reference_link_definitions_inner(link_defs)?;
                            // remove trailing space in case we're about to push newlines
                            *self.indentation.last_mut().unwrap() = ">".into();
                        } else {
                            write!(self, ">")?;
                            self.indentation.push(">".into());
                        }
                        // If there are any other trailing lines those should be handled by
                        // The End(BlockQuote) event.
                    }
                    Some((Event::Start(Tag::BlockQuote(_)), next_range)) => {
                        let snippet = &self.input[range.start..next_range.start];
                        let link_defs = parse_link_reference_definitions(snippet, range.start);

                        if link_defs.is_empty() {
                            write!(self, ">")?;
                            self.indentation.push(">".into());
                            let newlines = count_newlines(snippet);
                            self.write_newlines(newlines)?;
                        } else {
                            let end = link_defs.first().expect("we have link_defs").range().start;
                            let leading_newline_snippet = &self.input[range.start..end];
                            let newlines = count_newlines(leading_newline_snippet);

                            self.indentation.push("> ".into());
                            if newlines > 0 {
                                write!(self, ">")?;
                            } else {
                                write!(self, "> ")?;
                            }

                            self.rewrite_reference_link_definitions_inner(link_defs)?;
                        }
                    }
                    Some((_, next_range)) => {
                        let snippet = &self.input[range.start..next_range.start];
                        let link_defs = parse_link_reference_definitions(snippet, range.start);

                        let end = link_defs
                            .first()
                            .map(|l| l.range().start)
                            .unwrap_or(next_range.start);
                        let newline_snippet = &self.input[self.last_position..end];
                        let newlines = count_newlines(newline_snippet);

                        self.indentation.push("> ".into());
                        if newlines > 0 {
                            write!(self, ">")?;
                        } else {
                            write!(self, "> ")?;
                        }

                        if !link_defs.is_empty() {
                            self.rewrite_reference_link_definitions_inner(link_defs)?;
                        } else {
                            self.write_newlines(newlines)?;
                        }
                    }
                    None => {
                        // Peeking at the next event should always return `Some()` for start events
                        unreachable!("At the very least we'd expect an `End(BlockQuote)` event.");
                    }
                }
            }
            Tag::CodeBlock(kind) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                let capacity = (range.end - range.start) * 2;
                let code_block_buffer = String::with_capacity(capacity);
                match &kind {
                    CodeBlockKind::Fenced(info_string) => {
                        rewrite_marker(self.input, &range, self)?;

                        if info_string.is_empty() {
                            writeln!(self)?;
                            let writer = MarkdownWriter::CodeBlock((code_block_buffer, kind));
                            self.writers.push(writer);
                            return Ok(());
                        }

                        let marker_char = self.input[range.start..]
                            .chars()
                            .next()
                            .expect("should have found a ` or ~");

                        let starts_with_space = self.input[range.clone()]
                            .trim_start_matches(marker_char)
                            .starts_with(char::is_whitespace);

                        let info_string = self.input[range]
                            .lines()
                            .next()
                            .unwrap_or_else(|| info_string.as_ref())
                            .trim_start_matches(marker_char)
                            .trim();

                        if starts_with_space {
                            writeln!(self, " {info_string}")?;
                        } else {
                            writeln!(self, "{info_string}")?;
                        }
                    }
                    CodeBlockKind::Indented => {
                        // TODO(ytmimi) support tab as an indent
                        let indentation = self.indented_code_block_indentation();

                        if !matches!(self.peek(), Some(Event::End(TagEnd::CodeBlock))) {
                            // Only write indentation if this isn't an empty indented code block
                            self.write_str(indentation)?;
                        }

                        self.indentation.push(indentation.into());
                    }
                }

                let writer = MarkdownWriter::CodeBlock((code_block_buffer, kind));
                self.writers.push(writer);
            }
            Tag::List(_) => {
                if self.needs_indent {
                    // FIXME(ytmimi) The parser sometimes gets the ranges for a nested list
                    // wrong if it's preceded by a tab (\t) character
                    // See https://github.com/pulldown-cmark/pulldown-cmark/issues/983
                    let snippet = &self.input[range.clone()];
                    let special_case_list_start = snippet.starts_with(char::is_whitespace);

                    // Extra check to keep the output idempotent when we've got a nested list.
                    // Depending on how things get formated we might accidentally parse the
                    // paragraph and list start as a Setext header
                    let needs_extra_newline = |mut newlines: usize, range: Range<usize>| -> usize {
                        let last_was_paragraph =
                            matches!(self.last_event, Some((Event::End(TagEnd::Paragraph), _)));
                        let starts_with_dash = self.input[range].starts_with('-');
                        if newlines < 2 && last_was_paragraph && starts_with_dash {
                            newlines += 1;
                        }

                        newlines
                    };

                    let newlines = if special_case_list_start {
                        let start_offset = snippet.find(LIST_START_CHARS).unwrap_or(0);
                        let marker_start = range.start + start_offset;
                        let list_start_range = self.last_position..marker_start;
                        let newlines = self.count_newlines_in_range(&list_start_range);
                        needs_extra_newline(newlines, marker_start..range.end)
                    } else {
                        self.count_newlines(&range)
                    };

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

                // Anchor the last_position at the start of the blockquote. This prevents picking
                // up extra newlines in case we need to recover any refernce link definitions
                self.last_position = range.start;

                let is_empty_list = |snippet: &str| -> bool {
                    // It's an empty list if there are newlines between the list marker
                    // and the next event. For example,
                    //
                    // ```markdown
                    // -
                    //   foo
                    // ```
                    count_newlines(snippet) > 0
                };

                let list_marker = ListMarker::from_str(&self.input[range.clone()])
                    .expect("Should be able to parse a list marker");

                // FIXME(ytmimi) luckily recovering link-reference-definitions isn't overly
                // complicated for list items, but the implementations are very similar, so
                // hopefully there's some simple refactoring that can be done later.
                let (empty_list_item, link_defs) = match self.events.peek() {
                    Some((Event::End(TagEnd::Item), _)) => {
                        let snippet = &self.input[range.clone()];
                        let just_list_marker = snippet.trim().len() == list_marker.len();
                        let link_defs = parse_link_reference_definitions(snippet, range.start);
                        let end = link_defs
                            .first()
                            .map(|l| l.range().start)
                            .unwrap_or(range.end);
                        let snippet = &self.input[range.start..end];
                        (just_list_marker || is_empty_list(snippet), link_defs)
                    }
                    Some((_, next_range)) => {
                        let snippet = &self.input[range.start..next_range.start];
                        let link_defs = parse_link_reference_definitions(snippet, range.start);
                        let end = link_defs
                            .first()
                            .map(|l| l.range().start)
                            .unwrap_or(next_range.start);
                        let snippet = &self.input[range.start..end];
                        (is_empty_list(snippet), link_defs)
                    }
                    None => {
                        // Peeking at the next event should always return `Some()` for start events
                        unreachable!("At the very least we'd expect an `End(Item)` event.");
                    }
                };

                // We need to push a newline and indentation before the next event if
                // this is an empty list item
                self.needs_indent = empty_list_item;

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

                self.rewrite_reference_link_definitions_inner(link_defs)?;
            }
            Tag::FootnoteDefinition(ref label) => {
                let newlines = self.count_newlines(&range);
                self.write_newlines(newlines)?;

                // Anchor the last_position at the start of the footnote definition. This prevents
                // picking up extra newlines in case we need to recover refernce link definitions
                self.last_position = range.start;

                let recover_link_defs = |range: Range<usize>| -> Vec<LinkReferenceDefinition> {
                    let start = range.start;
                    let snippet = &self.input[range];
                    let colon_index = snippet.find(':').unwrap_or(0);
                    let snippet = &snippet[colon_index..];
                    parse_link_reference_definitions(snippet, start + colon_index)
                };

                let link_defs = match self.events.peek() {
                    Some((Event::End(TagEnd::FootnoteDefinition), _)) => {
                        recover_link_defs(range.clone())
                    }
                    Some((_, next_range)) => recover_link_defs(range.start..next_range.start),
                    None => {
                        // Peeking at the next event should always return `Some()` for start events
                        unreachable!(
                            "At the very least we'd expect an `End(FootnoteDefinition)` event."
                        );
                    }
                };

                write!(self, "[^{label}]:")?;

                let footnote = FootnoteDefinition::new(
                    // Take the indentaiton so that nested items are written without indentation.
                    // We will restore the indentation after we're done formatting the footnote def.
                    std::mem::take(&mut self.indentation),
                    (range.end - range.start) * 2,
                );

                self.writers.push(footnote.into());
                self.rewrite_reference_link_definitions_inner(link_defs)?;
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
            Tag::Link { link_type, .. } | Tag::Image { link_type, .. } => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                let capacity = (range.end - range.start) * 2;
                let is_auto_link = matches!(link_type, LinkType::Autolink | LinkType::Email);
                let link_writer = LinkWriter::new(capacity, is_auto_link);

                self.writers.push(link_writer.into());
                self.nested_context.push(tag);
            }
            Tag::Table(ref alignment) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                let table_state = TableState::new(alignment.clone());
                write!(self, "|")?;
                self.writers.push(table_state.into());
                self.indentation.push("|".into());
            }
            Tag::TableHead => {
                self.nested_context.push(tag);
            }
            Tag::TableRow => {
                self.nested_context.push(tag);
                if let Some(MarkdownWriter::Table(state)) = self.writers.last_mut() {
                    state.push_row()
                }
            }
            Tag::TableCell => {
                if !matches!(self.peek(), Some(Event::End(TagEnd::TableCell))) {
                    return Ok(());
                }

                if let Some(MarkdownWriter::Table(state)) = self.writers.last_mut() {
                    state.write(String::new().into());
                }
            }
            Tag::HtmlBlock => {
                // From what I've noticed leading whitespace isn't always parsed in definition lists
                let is_next_leading_whitespace =
                    matches!(self.peek(), Some(Event::Text(t)) if t.trim().is_empty());

                if self.in_definition_list_definition() && is_next_leading_whitespace {
                    // Because of how leading whitespace gets parsed for HTML blocks
                    // we need to update the definition list's indentation. so that the output
                    // stays idempotent.
                    if let Some(indent) = self.indentation.last_mut() {
                        *indent = " ".into();
                    }
                }

                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                }

                self.nested_context.push(tag);
                self.last_position = range.start;
            }
            Tag::MetadataBlock(_meta) => {
                let newlines = self.count_newlines(&range);
                self.write_newlines(newlines)?;
                rewrite_marker(self.input, &range, self)?;
                self.needs_indent = true;
            }
            Tag::DefinitionList => {
                let mut newlines = self.count_newlines(&range);

                if matches!(self.last_event, Some((Event::End(TagEnd::List(..)), _))) {
                    // At least two lines between the end of a list and the start of a
                    // definition list. This ensures that the output is idempotent.
                    newlines = std::cmp::max(newlines, 2);
                }

                self.write_newlines(newlines)?;
                self.nested_context.push(tag);
            }
            Tag::DefinitionListTitle => {
                let newlines = self.count_newlines(&range);
                self.write_newlines(newlines)?;
                let capacity = (range.end - range.start) * 2;
                let definition_list_title = DefinitionListTitle::new(capacity);
                self.writers.push(definition_list_title.into());
            }
            Tag::DefinitionListDefinition => {
                let newlines = self.count_newlines(&range);
                self.write_newlines(newlines)?;
                // Anchor the last_position at the start of the definition list definition.
                // This prevents picking up extra newlines in case we need to recover any refernce
                // link definitions
                self.last_position = range.start;

                // FIXME(ytmimi) `is_empty_list` is copied from `Tag::Item` above
                let is_empty_list = |snippet: &str| -> bool { count_newlines(snippet) > 0 };

                let (empty_definition_marker, force_newline_count, link_defs) = match self
                    .events
                    .peek()
                {
                    Some((Event::End(TagEnd::DefinitionListDefinition), _)) => {
                        let snippet = &self.input[range.clone()];
                        let link_defs = parse_link_reference_definitions(snippet, range.start);
                        let end = link_defs
                            .first()
                            .map(|l| l.range().start)
                            .unwrap_or(range.end);
                        let snippet = &self.input[range.start..end];
                        (is_empty_list(snippet) || link_defs.is_empty(), 0, link_defs)
                    }
                    Some((next_event, next_range)) => {
                        let snippet = &self.input[range.start..next_range.start];
                        let link_defs = parse_link_reference_definitions(snippet, range.start);
                        let end = link_defs
                            .first()
                            .map(|l| l.range().start)
                            .unwrap_or(next_range.start);
                        let snippet = &self.input[range.start..end];
                        let force_newlines = link_defs.is_empty()
                            && matches!(
                                next_event,
                                Event::Start(Tag::CodeBlock(CodeBlockKind::Indented))
                            );
                        let is_empty_list = is_empty_list(snippet);
                        if !is_empty_list && force_newlines {
                            (true, 1, link_defs)
                        } else {
                            (is_empty_list, 0, link_defs)
                        }
                    }
                    None => {
                        // Peeking at the next event should always return `Some()` for start events
                        unreachable!(
                            "At the very least we'd expect an `End(DefinitionListDefinition)` event"
                        );
                    }
                };

                self.empty_definition_list_definition_marker = empty_definition_marker;

                if empty_definition_marker {
                    write!(self, ":")?;
                } else {
                    write!(self, ": ")?;
                }

                self.nested_context.push(tag);
                self.indentation.push(DEFINITION_LIST_INDENTATION.into());
                if force_newline_count > 0 {
                    self.write_newlines(force_newline_count)?;
                }
                self.rewrite_reference_link_definitions_inner(link_defs)?;
            }
        }
        Ok(())
    }

    fn end_tag(&mut self, tag: TagEnd, range: Range<usize>) -> std::fmt::Result {
        match tag {
            TagEnd::Paragraph => {
                debug_assert!(matches!(
                    self.writers.last(),
                    Some(MarkdownWriter::Paragraph(_))
                ));
                let Some(MarkdownWriter::Paragraph(p)) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::Paragraph")
                };
                self.join_with_indentation(&p.into_buffer(), false)?;
            }
            TagEnd::Heading(_) => {
                debug_assert!(matches!(
                    self.writers.last(),
                    Some(MarkdownWriter::Header(_))
                ));
                let Some(MarkdownWriter::Header(h)) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::Header")
                };
                let header_kind = h.kind();
                let (buffer, indentation) = h.into_parts()?;

                if let HeaderKind::Atx(level) = header_kind {
                    let atx_header = match level {
                        HeadingLevel::H1 => "#",
                        HeadingLevel::H2 => "##",
                        HeadingLevel::H3 => "###",
                        HeadingLevel::H4 => "####",
                        HeadingLevel::H5 => "#####",
                        HeadingLevel::H6 => "######",
                    };

                    if buffer.is_empty() {
                        write!(self, "{}", atx_header.trim())?;
                    } else {
                        write!(self, "{atx_header} ")?;
                    }
                }
                self.indentation = indentation;
                self.join_with_indentation(&buffer, false)?;
            }
            TagEnd::BlockQuote(..) => {
                let ref_def_range = self.last_position..range.end;
                self.rewrite_reference_link_definitions(&ref_def_range)?;
                let rest_range = self.last_position..range.end;
                let newlines = self.count_newlines_in_range(&rest_range);
                if newlines > 0 {
                    // Recover empty block quote lines
                    if let Some(last) = self.indentation.last_mut() {
                        // Avoid trailing whitespace by replacing the last indentation with '>'
                        *last = ">".into()
                    }
                    self.write_newlines(newlines)?;
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.map(|t| t.to_end()), Some(tag));

                let popped_indentation = self
                    .indentation
                    .pop()
                    .expect("we pushed a blockquote marker in start_tag");
                if let Some(indentation) = self.indentation.last_mut() {
                    if indentation == ">" {
                        *indentation = popped_indentation
                    }
                }

                // FIXME(ytmimi) let chains would make this much nicer to write.
                //
                // If there's another event...
                if let Some((event, _)) = self.events.peek() {
                    // and it's not a TagEnd::BlockQuote...
                    if !matches!(event, Event::End(TagEnd::BlockQuote(_))) {
                        // and we still have indentation on the stack...
                        if let Some(last) = self.indentation.last_mut() {
                            // and the indentaion is ">"...
                            if last == ">" {
                                // update the indentation
                                *last = "> ".into()
                            }
                        }
                    }
                }
            }
            TagEnd::CodeBlock => {
                debug_assert!(matches!(
                    self.writers.last(),
                    Some(MarkdownWriter::CodeBlock(_))
                ));

                let Some(MarkdownWriter::CodeBlock((code_block, kind))) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::CodeBlock")
                };

                match kind {
                    CodeBlockKind::Fenced(info_string) => {
                        self.write_code_block_buffer(Some(info_string.as_ref()), code_block)?;
                        // write closing code fence
                        self.write_indentation(false)?;
                        rewrite_marker(self.input, &range, self)?;
                    }
                    CodeBlockKind::Indented => {
                        // Maybe we'll consider formatting indented code blocks??
                        self.write_code_block_buffer(None, code_block)?;

                        let popped_indentation = self
                            .indentation
                            .pop()
                            .expect("we added 4 spaces in start_tag");

                        let expected_indentation = self.indented_code_block_indentation();
                        debug_assert_eq!(popped_indentation, expected_indentation);
                    }
                }
            }
            TagEnd::List(_) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.map(|t| t.to_end()), Some(tag));
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
            TagEnd::Item => {
                let ref_def_range = self.last_position..range.end;
                self.rewrite_reference_link_definitions(&ref_def_range)?;
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines_no_trailing_whitespace(newlines)?;
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.map(|t| t.to_end()), Some(tag));
                let popped_indentation = self.indentation.pop();
                debug_assert!(popped_indentation.is_some());

                // if the next event is a Start(Item), then we need to set needs_indent
                self.needs_indent = matches!(self.peek(), Some(Event::Start(Tag::Item)));
            }
            TagEnd::FootnoteDefinition => {
                debug_assert!(matches!(
                    self.writers.last(),
                    Some(MarkdownWriter::FootnoteDefinition(_))
                ));

                let ref_def_range = self.last_position..range.end;
                self.rewrite_reference_link_definitions(&ref_def_range)?;

                let Some(MarkdownWriter::FootnoteDefinition(f)) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::FootnoteDefinition")
                };

                let (buffer, indentation, footnote_indent) = f.into_parts();
                self.indentation = indentation;

                if !buffer.is_empty() {
                    writeln!(self)?;
                    self.indentation.push(footnote_indent);
                    self.join_with_indentation(&buffer, true)?;
                    self.indentation.pop();
                }

                if let Some(Event::Start(Tag::FootnoteDefinition(_))) = self.peek() {
                    // separte consecutive footnote definitinons by at least one line
                    self.write_newlines(1)?;
                };
            }
            TagEnd::Emphasis => {
                rewrite_marker_with_limit(self.input, &range, self, Some(1))?;
            }
            TagEnd::Strong => {
                rewrite_marker_with_limit(self.input, &range, self, Some(2))?;
            }
            TagEnd::Strikethrough => {
                rewrite_marker(self.input, &range, self)?;
            }
            TagEnd::Link | TagEnd::Image => {
                debug_assert!(matches!(self.writers.last(), Some(MarkdownWriter::Link(_))));

                let Some(MarkdownWriter::Link(link_writer)) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::Link")
                };

                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref().map(|t| t.to_end()), Some(tag));

                let (link_type, url, title) = match popped_tag {
                    Some(
                        ref tag @ Tag::Link {
                            ref link_type,
                            ref dest_url,
                            ref title,
                            ..
                        },
                    ) => {
                        let email_or_auto =
                            matches!(link_type, LinkType::Email | LinkType::Autolink);
                        let opener = if email_or_auto { "<" } else { "[" };
                        self.write_tag_str(&tag, opener)?;
                        (link_type, dest_url, title)
                    }
                    Some(Tag::Image {
                        ref link_type,
                        ref dest_url,
                        ref title,
                        ..
                    }) => {
                        write!(self, "![")?;
                        (link_type, dest_url, title)
                    }
                    _ => {
                        panic!("Expected a Tag::Link or Tag::Image")
                    }
                };
                self.write_str(&link_writer.into_buffer())?;
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
                            self.write_inline_link(&url, title)?;
                        }
                    }
                    LinkType::Reference | LinkType::ReferenceUnknown => {
                        let label = crate::links::find_reference_link_label(text);
                        if count_newlines(label) > 0 {
                            write!(self, "][")?;
                            let label = split_lines(label)
                                .map(|l| self.trim_leading_indentation(l))
                                .join("\n");

                            if label.starts_with('^') {
                                write!(self, "\\")?;
                            }

                            self.write_str(&label)?;
                            write!(self, "]")?;
                        } else if label.starts_with('^') {
                            write!(self, "][\\{label}]")?;
                        } else {
                            write!(self, "][{label}]")?;
                        }
                    }
                    LinkType::Collapsed | LinkType::CollapsedUnknown => write!(self, "][]")?,
                    LinkType::Shortcut | LinkType::ShortcutUnknown => {
                        write!(self, "]")?;

                        if let Some((Event::Text(text), next_range)) = self.events.peek() {
                            let between_snippet = &self.input[range.end..next_range.start];
                            let needs_escape = !sequence_ends_on_escape(between_snippet);
                            if text.starts_with(':') && needs_escape {
                                write!(self, "\\")?;
                            }
                        }
                    }
                    LinkType::Autolink | LinkType::Email => write!(self, ">")?,
                }
            }
            TagEnd::Table => {
                debug_assert!(matches!(
                    self.writers.last(),
                    Some(MarkdownWriter::Table(_))
                ));

                let Some(MarkdownWriter::Table(t)) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::Table")
                };
                self.join_with_indentation(&t.format()?, false)?;
                let popped_indentation = self.indentation.pop().expect("we added `|` in start_tag");
                debug_assert_eq!(popped_indentation, "|");
            }
            TagEnd::TableRow | TagEnd::TableHead => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.map(|t| t.to_end()), Some(tag));
            }
            TagEnd::TableCell => {
                if let Some(MarkdownWriter::Table(state)) = self.writers.last_mut() {
                    // We finished formatting this cell. Setup the state to format the next cell
                    state.increment_col_index()
                }
            }
            TagEnd::HtmlBlock => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref().map(|t| t.to_end()), Some(tag));

                if self.in_definition_list_definition() {
                    // Restore the indentation that we modified in `Tag::HtmlBlock`.
                    if let Some(indent) = self.indentation.last_mut() {
                        *indent = DEFINITION_LIST_INDENTATION.into();
                    }
                }
            }
            TagEnd::MetadataBlock(_meta) => {
                rewrite_marker(self.input, &range, self)?;
                self.needs_indent = true;
            }
            TagEnd::DefinitionList => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref().map(|t| t.to_end()), Some(tag));
            }
            TagEnd::DefinitionListTitle => {
                debug_assert!(matches!(
                    self.writers.last(),
                    Some(MarkdownWriter::DefinitionListTitle(_))
                ));
                let Some(MarkdownWriter::DefinitionListTitle(d)) = self.writers.pop() else {
                    unreachable!("Should have popped a MarkdownWriter::DefinitionListTitle")
                };

                let buffer = d.into_buffer();
                if needs_escape(&buffer).is_some() {
                    self.write_str("\\")?;
                }
                self.join_with_indentation(&buffer, false)?;
            }
            TagEnd::DefinitionListDefinition => {
                let ref_def_range = self.last_position..range.end;
                self.rewrite_reference_link_definitions(&ref_def_range)?;

                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref().map(|t| t.to_end()), Some(tag));

                let popped_indentation = self
                    .indentation
                    .pop()
                    .expect(r#"we added "  " in Tag::DefinitionListDefinition"#);
                debug_assert_eq!(popped_indentation, DEFINITION_LIST_INDENTATION);

                let rest_range = self.last_position..range.end;
                let newlines = self.count_newlines_in_range(&rest_range);
                if newlines > 0 {
                    // Recover empty lines at the end of the definition list
                    self.write_newlines_no_trailing_whitespace(newlines)?;
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
        writer.write_str(&marker[..mark_max_width])
    } else {
        writer.write_str(marker)
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
