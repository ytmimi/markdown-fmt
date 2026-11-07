use std::collections::VecDeque;
use crate::events::{MdEvent, Custom};
use crate::links::{LinkReferenceDefinition, parse_link_reference_definitions};
use pulldown_cmark::{Event, Tag};

/// Conveniently turn any iterator that returns MdEvent into a LinkReferenceDefinitionInjector
pub(crate) trait LinkReferenceDefinitionEventExt<'input, I>
where
    I: Iterator<Item = MdEvent<'input>>
{
    fn inject_list_reference_definition_events(self, input: &'input str) -> LinkReferenceDefinitionInjector<'input, I>;
}

// Blanket impl for all (Event<'input>, std::ops::Range<usize>) Iterators
impl<'input, I> LinkReferenceDefinitionEventExt<'input, I> for I
where
    I: Iterator<Item = MdEvent<'input>>
{
    fn inject_list_reference_definition_events(self, input: &'input str) -> LinkReferenceDefinitionInjector<'input, I> {
        LinkReferenceDefinitionInjector {
            inner: self,
            input,
            last_event: None,
            definitions: VecDeque::new(),
            last_position: 0,
            done: false,
        }
    }
}

/// Add Custom LinkReferenceDefinition events to the event stream
pub(crate) struct LinkReferenceDefinitionInjector<'input, I> {
    inner: I,
    input: &'input str,
    last_event: Option<MdEvent<'input>>,
    definitions: VecDeque<(LinkReferenceDefinition<'input>, std::ops::Range<usize>)>,
    last_position: usize,
    done: bool,
}

impl<'input, I> Iterator for LinkReferenceDefinitionInjector<'input, I>
where
    I: Iterator<Item = MdEvent<'input>>
{
    type Item = MdEvent<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        // First, check if we have any definitions to emit
        if let Some((d, r)) = self.definitions.pop_front() {
            self.last_position = std::cmp::max(self.last_position, r.end);
            tracing::debug!("Returning parsed definition: {d:?}");
            return Some(MdEvent::Custom(Custom::LinkReferenceDefinition(d), r));
        }

        // Second, check if we've got a stored event that needs to be returned
        if let Some(MdEvent::Standard(_, ref r) | MdEvent::Custom(_, ref r)) = self.last_event {
            self.last_position = std::cmp::max(self.last_position, r.end);
            return self.last_event.take();
        }

        if self.done {
            // We know there are no more events to return
            return None;
        }

        // Third, check if we need to parse out any definitions
        let event = self.inner.next();
        match &event {
            None => {
                // There are no more events. Lets see if we need to parse out any link
                // reference definitions at the end of the file
                eprintln!("Parsing definition from {:?}", &self.input[self.last_position..]);
                let defs = parse_link_reference_definitions(&self.input[self.last_position..], self.last_position);
                if !defs.is_empty() {
                    for (_, r) in defs.iter()  {
                        debug_assert!(!self.input[r.clone()].is_empty(), "Can't have an empty link reference definition");
                    }
                    eprintln!("Parsed definitions {defs:?}");
                }
                self.definitions.extend(defs);
                self.done = true;
            }
            Some(MdEvent::Standard(Event::Start(tag), range)) => {
                if !matches!(tag, Tag::TableHead | Tag::TableRow | Tag::TableCell) {
                    // The next event is a start event. Let's see if we can find any link reference definitions
                    // between the last event and the start event
                    eprintln!("Parsing definition from {:?}", &self.input[self.last_position..range.start]);
                    let defs = parse_link_reference_definitions(&self.input[self.last_position..range.start], self.last_position);
                    if !defs.is_empty() {
                        eprintln!("Parsed definitions {defs:?}");
                    }
                    self.definitions.extend(defs);
                    // Bump `self.last_position` to `range.start` so that we can keep track of
                    // what we've alredy seen.
                    self.last_position = range.start;
                }
            }
            Some(MdEvent::Standard(_, ref range) | MdEvent::Custom(_, ref range)) => {
                // Some other event. Let's just keep track of where the event ends
                self.last_position = range.end;
            }
        };

        // If we successfully parsed link reference definitions above return that instead
        if let Some((d, r)) = self.definitions.pop_front() {
            self.last_position = std::cmp::max(self.last_position, r.end);
            self.last_event = event;
            eprintln!("Found this definition: {:?}", &self.input[r.clone()]);
            return Some(MdEvent::Custom(Custom::LinkReferenceDefinition(d), r));
        } else {
            event
        }
    }
}