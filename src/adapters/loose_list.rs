use pulldown_cmark::{Event, Tag, TagEnd};
use std::collections::VecDeque;
use std::iter::Peekable;

/// Conveniently turn any iterator that returns (Event, Range) into a LooseListAdapter
pub(crate) trait LooseListExt<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn all_loose_lists(self) -> LooseListAdapter<'input, I>;
}

// Blanket impl for all iterators
impl<'input, I> LooseListExt<'input, I> for I
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn all_loose_lists(self) -> LooseListAdapter<'input, I> {
        LooseListAdapter::new(self)
    }
}

/// Converts [`tight`] lists into [`loose`] lists.
///
/// A loose list contains items that are separated by newlines.
/// See <https://spec.commonmark.org/0.30/#list> fot more about `tight` vs `loose` lists.
///
/// [`tight`]: https://spec.commonmark.org/0.30/#tight
/// [`loose`]: https://spec.commonmark.org/0.30/#loose
///
/// ## Loose List
///
/// ```markdown
/// * a
///
///   b
/// ```
///
/// The snippet would generate the following Markdown Events
///
/// ```text
/// Start(List(None))
/// event=Start(Item)
/// event=Start(Paragraph)
/// event=Text(Borrowed("a"))
/// event=End(Paragraph)
/// event=Start(Paragraph)
/// event=Text(Borrowed("b"))
/// event=End(Paragraph)
/// event=End(Item)
/// event=End(List(None))
/// ```
///
/// ## Tight List
///
/// ```markdown
/// * c
///   d
/// ```
///
/// This snippet generates the following Markdown Events. **Note** that there aren't any
/// `Paragraph` events, and that's how we know the list item is tight.
///
/// ```text
/// event=Start(List(None))
/// event=Start(Item)
/// event=Text(Borrowed("c"))
/// event=SoftBreak
/// event=Text(Borrowed("d"))
/// event=End(Item)
/// event=End(List(None))
/// ```
///
/// ## Transformation
///
/// The goal of the [LooseListAdapter] is to convert all tight list items into loose list items
/// by wrapping them in paragraph events.
///
/// ```markdown
/// * e
///   f
/// ```
///
/// The adapter will generate the following events
///
/// ```text
/// event=Start(List(None))
/// event=Start(Item)
/// event=Start(Paragraph)
/// event=Text(Borrowed("c"))
/// event=SoftBreak
/// event=Text(Borrowed("d"))
/// event=End(Paragraph)
/// event=End(Item)
/// event=End(List(None))
/// ```
pub(crate) struct LooseListAdapter<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    /// Inner iterator that return Events
    inner: Peekable<I>,
    /// Represents offsets into the intermediate_events
    loose_list_stack: Vec<Option<usize>>,
    /// Keeps track of events until we're ready to start returning them.
    stashed_events: VecDeque<(Event<'input>, std::ops::Range<usize>)>,
}

impl<'input, I> LooseListAdapter<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    pub(super) fn new(inner: I) -> Self {
        Self {
            inner: inner.peekable(),
            loose_list_stack: vec![],
            stashed_events: VecDeque::new(),
        }
    }
    /// Check if the next event denotes a tight list.
    ///
    /// this should only be called when in the context of a list item.
    fn is_tight_list(event: &Event<'_>) -> bool {
        match event {
            Event::Text(_)
            | Event::Code(_)
            | Event::FootnoteReference(_)
            | Event::TaskListMarker(_)
            | Event::InlineHtml(_)
            | Event::Start(Tag::Link { .. }) => true,
            Event::Html(text) => is_single_html_tag(text),
            _ => false,
        }
    }
}

/// Check if the HTML content is a single tag. For example, `<{tag}>` or `</{tag}>`.
fn is_single_html_tag(html: &str) -> bool {
    // A hacky way to figure out if this is an inline html tag, but I think it works!
    html.starts_with('<')
        && html.ends_with('>')
        && html.bytes().filter(|b| matches!(b, b'<' | b'>')).count() == 2
}

macro_rules! push_end_paragraph {
    ($index:expr, $stashed_events:expr, $end:expr) => {
        if let Some((paragraph, paragraph_range)) = $stashed_events.get_mut($index) {
            debug_assert!(matches!(paragraph, Event::Start(Tag::Paragraph)));

            let full_range = paragraph_range.start..$end;
            *paragraph_range = full_range.clone();

            let paragraph_end = Event::End(TagEnd::Paragraph);
            $stashed_events.push_back((paragraph_end, full_range))
        } else {
            panic!("We should have stashed a Start(Paragraph) event");
        }
    };
}

macro_rules! maybe_push_start_paragraph {
    ($self:expr, $name:ident) => {
        // FIXME(ytmimi) let-chains would make this nicer to write
        if let Some(last @ None) = $self.loose_list_stack.last_mut() {
            if let Some((next_event, next_range)) = $self.inner.peek() {
                if $name::is_tight_list(next_event) {
                    let index = $self.stashed_events.len();
                    *last = Some(index);

                    // produce a synthetic `Start(Paragraph)` event.
                    $self
                        .stashed_events
                        .push_back((Event::Start(Tag::Paragraph), next_range.clone()));
                }
            }
        }
    };
}

impl<'input, I> Iterator for LooseListAdapter<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    type Item = (Event<'input>, std::ops::Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if let next @ Some(_) = self.stashed_events.pop_front() {
            return next;
        }

        loop {
            let (current_event, current_range) = match self.inner.next() {
                Some(current) => current,
                None => {
                    return self.stashed_events.pop_front();
                }
            };

            tracing::debug!(event=?current_event, range=?current_range);

            match current_event {
                Event::Start(Tag::Item) => {
                    self.stashed_events
                        .push_back((current_event, current_range));

                    // FIXME(ytmimi) let-chains would make this nicer to write
                    match self.inner.peek() {
                        Some((next_event, next_range)) if Self::is_tight_list(next_event) => {
                            // Use this index later to update the `Start(Paragraph)`'s end range
                            let index = self.stashed_events.len();
                            self.loose_list_stack.push(Some(index));

                            // produce a synthetic `Start(Paragraph)` event.
                            self.stashed_events
                                .push_back((Event::Start(Tag::Paragraph), next_range.clone()));
                        }
                        _ => {
                            self.loose_list_stack.push(None);
                        }
                    }
                }
                Event::End(TagEnd::Item) => {
                    self.stashed_events
                        .push_back((current_event, current_range));

                    if self.loose_list_stack.is_empty() {
                        // Once the stack is empty it's fine to start returning events that have
                        // been stashed. Trying to return before this point would lead to errors,
                        // becuase we the loose_list_stack holds indexes into the stashed_events.
                        return self.stashed_events.pop_front();
                    }
                }
                Event::End(
                    TagEnd::Heading(..)
                    | TagEnd::List(_)
                    | TagEnd::BlockQuote
                    | TagEnd::CodeBlock
                    | TagEnd::Table
                    | TagEnd::HtmlBlock
                    | TagEnd::FootnoteDefinition,
                ) => {
                    self.stashed_events
                        .push_back((current_event, current_range));
                    maybe_push_start_paragraph!(self, Self);
                }
                Event::Html(ref text) if !is_single_html_tag(text) => {
                    self.stashed_events
                        .push_back((current_event, current_range));
                    maybe_push_start_paragraph!(self, Self);
                }
                _ => {
                    self.stashed_events
                        .push_back((current_event, current_range.clone()));

                    if self.loose_list_stack.is_empty() {
                        return self.stashed_events.pop_front();
                    }

                    // Match on events that could interrupt a paragraph, and if we're currently
                    // converting a "tight" list to a "loose" list, then push an `End(Paragraph)`
                    match self.inner.peek() {
                        Some((Event::End(TagEnd::Item), _)) => {
                            // NOTE that we pop off of the loose_list_stack instead of peeking at
                            // it like we do in the other arms
                            if let Some(Some(index)) = self.loose_list_stack.pop() {
                                push_end_paragraph!(index, self.stashed_events, current_range.end)
                            }
                        }
                        Some((
                            Event::Start(
                                Tag::Heading { .. }
                                | Tag::List(_)
                                | Tag::BlockQuote(_)
                                | Tag::CodeBlock(_)
                                | Tag::Table(_)
                                | Tag::HtmlBlock
                                | Tag::FootnoteDefinition(_),
                            ),
                            _,
                        )) => {
                            if let Some(Some(index)) =
                                self.loose_list_stack.last_mut().map(|i| i.take())
                            {
                                push_end_paragraph!(index, self.stashed_events, current_range.end)
                            }
                        }
                        Some((Event::Html(ref html), _)) if !is_single_html_tag(html) => {
                            if let Some(Some(index)) =
                                self.loose_list_stack.last_mut().map(|i| i.take())
                            {
                                push_end_paragraph!(index, self.stashed_events, current_range.end)
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::FormatBuilder;
    use crate::formatter::FormatState;
    use crate::test::get_test_files;
    use std::borrow::Cow;
    use std::path::PathBuf;

    macro_rules! check_unchanged_events {
        ($markdown:literal) => {
            let options = crate::pulldown_cmark_options!();

            let input = pulldown_cmark::Parser::new_ext($markdown, options.clone())
                .into_offset_iter()
                .collect::<Vec<_>>();
            let output = pulldown_cmark::Parser::new_ext($markdown, options)
                .into_offset_iter()
                .all_loose_lists()
                .collect::<Vec<_>>();

            assert_eq!(input, output)
        };
    }

    #[test]
    fn check_unchanged() {
        check_unchanged_events!("");
        check_unchanged_events!("a");
        check_unchanged_events!(">");
        check_unchanged_events!("*");
        check_unchanged_events!("+");
        check_unchanged_events!("-");
        check_unchanged_events!("> *");
        check_unchanged_events!("> +");
        check_unchanged_events!("> -");
        check_unchanged_events!("# header");
        check_unchanged_events!("* ## header");
        check_unchanged_events!("+ ## header");
        check_unchanged_events!("- ## header");
        check_unchanged_events!(
            r"
- ```text
  ```
"
        );
        check_unchanged_events!(
            r"
- ```text
  ```
  >
  -
  ```text
  another code block
  ```
 <div>block level html is also unchanged</div>
"
        );

        check_unchanged_events!(
            r"
-
  -
    -
      -
"
        );
        check_unchanged_events!(
            r"
| col 1 | col 2 |
| ----- | ----- |
| 1     | 2     |
"
        );

        check_unchanged_events!(
            r"
- | table | heading is longer than content (in list) |
  | ----- | ---------------------------------------- |
  | val   | x                                        |
"
        );

        check_unchanged_events!("    indent code block");
        check_unchanged_events!(
            r"
* a

  b
"
        );

        check_unchanged_events!(
            r#"
* This is Already a loose list

  <span>some inline </span> html.

  ```rust
  fn main() {
    println!("hello world!");
  }
  ```
"#
        );
    }

    const SEPARATOR: &str = "==========";

    /// Make sure that the adapter generates reasonable events when converting tight lists
    /// into loose lists.
    #[test]
    fn check_adapter_events() {
        let mut file = PathBuf::from(std::file!());
        file.pop();
        let test_dir = file.join("test/loose_list");
        for file in get_test_files(test_dir, "txt") {
            let content = std::fs::read_to_string(&file).unwrap();
            let (markdown, expected_events) = content.split_once(SEPARATOR).unwrap();

            let options = crate::pulldown_cmark_options!();

            let events = pulldown_cmark::Parser::new_ext(markdown, options)
                .into_offset_iter()
                .all_loose_lists();

            if std::env::var("GENERATE_EVENTS").is_ok() {
                // write the events out to the file
                let mut output: Vec<Cow<'_, str>> = vec![
                    markdown.trim().into(),
                    "".into(),
                    SEPARATOR.into(),
                    "".into(),
                ];

                for (event, range) in events {
                    output.push(format!("event={event:?} range={range:?}").into())
                }
                output.push("".into());
                std::fs::write(file, &output.join("\n")).unwrap();
            } else {
                for ((event, range), line) in events.zip(expected_events.trim().lines()) {
                    assert_eq!(format!("event={event:?} range={range:?}"), line);
                }
            }

            // Get the events before formatting
            let pre_events = pulldown_cmark::Parser::new_ext(markdown, options).collect::<Vec<_>>();

            // Use the `all_loose_lists` adapter when formatting
            let adapted_events = pulldown_cmark::Parser::new_ext(markdown, options)
                .into_offset_iter()
                .all_loose_lists();

            let formatter = FormatBuilder::default().build();
            let fmt_state = FormatState::new(markdown, &formatter, adapted_events);

            let output = fmt_state.format().unwrap();

            // Get the events after formatting
            let pos_events = pulldown_cmark::Parser::new_ext(&output, options).collect::<Vec<_>>();

            // Make sure the events haven't changed after formatting
            assert_eq!(pre_events, pos_events)
        }
    }
}
