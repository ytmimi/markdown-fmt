//! Something changed in pulldown-cmark v0.12.0 that messed with the `range.end` for lists.
//!
//! This module helps fix that issue by pinning the end of the list to it's last item's `range.end`.

use pulldown_cmark::{Event, TagEnd};

/// Conveniently turn any iterator that returns (Event, Range) into a ListEndAtLastItemAdapter
pub(crate) trait ListEndAtLastItemExt<'input, I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn list_end_at_last_item(self) -> ListEndAtLastItemAdapter<I>;
}

// Blanket impl for all (Event<'input>, std::ops::Range<usize>) Iterators
impl<'input, I> ListEndAtLastItemExt<'input, I> for I
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    fn list_end_at_last_item(self) -> ListEndAtLastItemAdapter<I> {
        ListEndAtLastItemAdapter {
            end_position: 0,
            inner: self,
        }
    }
}

/// Iterator Adapter that modifies the source range of all `TagEnd::List` events to end at the
/// same location as their last list item. This helps us preserve newlines between the end of
/// a list and whatever element comes next.
pub(crate) struct ListEndAtLastItemAdapter<I> {
    end_position: usize,
    inner: I,
}

impl<'input, I> Iterator for ListEndAtLastItemAdapter<I>
where
    I: Iterator<Item = (Event<'input>, std::ops::Range<usize>)>,
{
    type Item = (Event<'input>, std::ops::Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (event, mut range) = self.inner.next()?;
        match event {
            Event::End(TagEnd::Item | TagEnd::DefinitionListDefinition) => {
                // `TagEnd::Item` should always precede `TagEnd::List` and
                // `TagEnd::DefinitionListDefinition` should always precede `TagEnd::DefinitionList`
                self.end_position = range.end
            }
            Event::End(TagEnd::List(..) | TagEnd::DefinitionList) => {
                // Just update the current `TagEnd::List` with the last `TagEnd::Item` range.end or
                // update the TagEnd::DefinitionList with the TagEnd::DefinitionListDefinition end
                range.end = self.end_position
            }
            _ => {}
        }
        Some((event, range))
    }
}
