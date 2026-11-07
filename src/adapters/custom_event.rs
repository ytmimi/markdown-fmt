use crate::events::MdEvent;
use pulldown_cmark::Event;

/// Conveniently turn any iterator that returns (Event, Range) into a CustomEventAdapter
pub(crate) trait MarkdownEventExt<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn md_events(self) -> MarkdownEventAdapter<I>;
}

// Blanket impl for all (Event<'input>, std::ops::Range<usize>) Iterators
impl<'input, I> MarkdownEventExt<'input, I> for I
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn md_events(self) -> MarkdownEventAdapter<I> {
        MarkdownEventAdapter { inner: self }
    }
}

pub(crate) struct MarkdownEventAdapter<I> {
    inner: I,
}

impl<'input, I> Iterator for MarkdownEventAdapter<I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    type Item = MdEvent<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let (e, r) = self.inner.next()?;
        Some(MdEvent::Standard(e, r))
    }
}
