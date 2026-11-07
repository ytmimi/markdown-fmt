use pulldown_cmark::Event;
use crate::links::LinkReferenceDefinition;

/// Some Markdown Event 
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MdEvent<'input> {
    // Regular event parsed from the pulldown cmark parser
    Standard(Event<'input>, std::ops::Range<usize>),
    // Custom events that ties back to the source file.
    // This represents events that pulldown cmark doesn't like link reference definitions,
    // but could also be used to reinterpret pulldown cmark events for better round trip formatting.
    Custom(Custom<'input>, std::ops::Range<usize>),
}

/// Custom Events defined by the Markdown Formatter
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Custom<'input> {
    LinkReferenceDefinition(LinkReferenceDefinition<'input>)
}