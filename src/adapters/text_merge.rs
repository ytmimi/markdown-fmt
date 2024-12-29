use crate::links::is_balanced;
use pulldown_cmark::{CowStr, Event, Tag, TagEnd};

/// Conveniently turn any iterator that returns (Event, Range) into a MergeBracketedParagraphText
pub(crate) trait MergeBracketedTextExt<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn merge_bracketed_text(self, input: &'input str) -> MergeBracketedParagraphText<'input, I>;
}

// Blanket impl for all iterators
impl<'input, I> MergeBracketedTextExt<'input, I> for I
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn merge_bracketed_text(self, input: &'input str) -> MergeBracketedParagraphText<'input, I> {
        MergeBracketedParagraphText::new(input, self)
    }
}

/// Similar functionality to [pulldown_cmark::util::TextMergeWithOffset], but only merges text
/// that begins with `[`.
///
/// The parser is a bit peculiar in how it returns text events. Key characters like `*`, `[`, `]`
/// are usually parsed on their own. By grouping bracketed text we can run additional checks on the
/// text to see whether or not it resembles other markdown constructs that should be escaped.
/// For example, footnote references.
pub(crate) struct MergeBracketedParagraphText<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    inner: I,
    last_event: Option<I::Item>,
    input: &'input str,
    in_paragraph: bool,
}

impl<'input, I> MergeBracketedParagraphText<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    pub(super) fn new(input: &'input str, inner: I) -> Self {
        Self {
            inner,
            input,
            last_event: None,
            in_paragraph: false,
        }
    }

    fn update_paragraph_status(&mut self, event: Option<&(Event<'input>, std::ops::Range<usize>)>) {
        if matches!(event, Some((Event::Start(Tag::Paragraph), _))) {
            self.in_paragraph = true
        }

        if matches!(event, Some((Event::End(TagEnd::Paragraph), _))) {
            self.in_paragraph = false
        }
    }
}

impl<'input, I> Iterator for MergeBracketedParagraphText<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let last_event = self.last_event.take();
        let next_event = self.inner.next();

        // This implementation is nearly identical to `pulldown_cmark::util::TextMergeWithOffset`,
        // but with modifications to only merge paragraph text. The assumption here is that
        // paragraph text is always consecutive and newlines within paragraphs will result in
        // `Event::Softbreak` and `Event::Hardbreak` events breaking up the consecutive text.
        match (last_event, next_event) {
            (
                Some((Event::Text(last_text), last_offset)),
                Some((Event::Text(next_text), next_offset)),
            ) if self.in_paragraph && last_text.starts_with('[') => {
                tracing::debug!(
                    "Start merging text events with:\n\tlast: {last_text:?}\n\tnext: {next_text:?}"
                );
                // We need to start merging consecutive text events together into one
                let mut offset = last_offset.clone();
                offset.end = next_offset.end;
                loop {
                    let current_text = &self.input[offset.clone()];
                    match self.inner.next() {
                        Some((Event::Text(ref t), ref next_offset))
                            if !is_balanced(current_text, '[', ']') =>
                        {
                            tracing::debug!("Continue merging text events:\n\tnext: {t:?}");
                            offset.end = next_offset.end;
                        }
                        next_event => {
                            self.last_event = next_event;
                            let output = &self.input[offset.clone()];
                            let event = Some((Event::Text(CowStr::Borrowed(output)), offset));
                            tracing::debug!(
                                "Finish merging text events:\n\tFinal event: {event:?}"
                            );
                            break event;
                        }
                    }
                }
            }
            (None, next_event @ Some(_)) => {
                tracing::debug!(last_event="None", next_event=?next_event);
                // This happens on the first iteration when the iterator has items.
                // Check if we started with a paragrph, then update the `last_event` so that
                // future iterations will return the `last_event` and finally return the next_event
                self.update_paragraph_status(next_event.as_ref());
                self.last_event = self.inner.next();
                next_event
            }
            (None, None) => {
                tracing::debug!(last_event = "None", next_event = "None");
                // This happens when the iterator is depleted
                None
            }
            (last_event, next_event) => {
                tracing::debug!(last_event=?last_event, next_event=?next_event);
                // The common case, emit the `last_event` and update which event we'll produce next.
                self.update_paragraph_status(last_event.as_ref());
                self.last_event = next_event;
                last_event
            }
        }
    }
}
