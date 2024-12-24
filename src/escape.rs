use super::formatter::FormatState;
use crate::list::UnorderedListMarker;
use pulldown_cmark::Event;

impl<'i, I> FormatState<'i, '_, I>
where
    I: Iterator<Item = (Event<'i>, std::ops::Range<usize>)>,
{
    pub(super) fn needs_escape(
        &mut self,
        input: &str,
        is_inline_element: bool,
    ) -> Option<EscapeKind> {
        let first_char = input.chars().next()?;

        if !is_inline_element {
            if !self.last_was_softbreak() {
                // Normally, we only need to escape after a softbreak since the markdown
                // formatter will adjust indentation, and the semantics of the formatted markdown
                // could change if we don't escape values. Because different markdown constructs
                // have higher or lower precedence there are cases where we want to aggresively
                // escape characters to avoid the formatted code from being parsed differently
                // on future formatting runs.
                //
                // As an example, see <https://spec.commonmark.org/0.30/#example-70> as a case
                // where the semantics would change without an escape after removing indentation.
                return None;
            }

            // Don't interpret the `>` as a blockquote
            if first_char == '>' {
                return Some(EscapeKind::SingleLine(SingleLineEscape::BlockQuote));
            }

            if input.len() <= 2 {
                return None;
            }
        }

        needs_escape(input)
    }
}

pub(crate) fn needs_escape(input: &str) -> Option<EscapeKind> {
    let first_char = input.chars().next()?;

    let marker_with_optional_trailing_space = |max_marker_count: usize, marker: char| -> bool {
        let mut leading_marker_count = 0;
        let mut whitespace_after_marker = false;
        for c in input.chars() {
            if c == marker {
                leading_marker_count += 1;
                continue;
            }

            if c.is_whitespace() {
                whitespace_after_marker = true;
            }

            break;
        }

        let empty_marker = || input.chars().all(|c| c == marker);

        leading_marker_count <= max_marker_count && (whitespace_after_marker || empty_marker())
    };
    let is_atx_heading = || marker_with_optional_trailing_space(6, '#');
    let is_setext_heading = |value: u8| input.trim_end().bytes().all(|b| b == value);
    let is_unordered_list_marker = |value: char| marker_with_optional_trailing_space(1, value);
    let is_thematic_break = |value: u8| {
        input.bytes().all(|b| b == value || b == b' ')
            && input.bytes().filter(|b| *b == value).count() >= 3
    };
    let is_fenced_code_block = |value: &str| {
        let marker = value
            .chars()
            .next()
            .expect("value hast at least one character");

        if !input.starts_with(value) {
            return false;
        }

        let info_string = input.trim_start_matches(marker).trim_start();

        if info_string.is_empty() {
            return true;
        }

        // opening code fences can't contain backtick (`) or tilde (~) in the info string.
        !info_string.contains(['`', '~'])
    };

    match first_char {
        '#' if is_atx_heading() => Some(EscapeKind::SingleLine(SingleLineEscape::AtxHeader)),
        '=' if is_setext_heading(b'=') => {
            let marker = SetextHeaderMarker::H1;
            Some(EscapeKind::MultiLine(MultiLineEscape::SetextHeader(marker)))
        }
        '-' => {
            if is_thematic_break(b'-') {
                let escape = SingleLineEscape::ThematicBreak(ThematicBreakMarker::Hyphen);
                Some(EscapeKind::SingleLine(escape))
            } else if is_unordered_list_marker('-') {
                let escape = SingleLineEscape::UnorderedList(UnorderedListMarker::Hyphen);
                Some(EscapeKind::SingleLine(escape))
            } else if is_setext_heading(b'-') {
                let escape = MultiLineEscape::SetextHeader(SetextHeaderMarker::H2);
                Some(EscapeKind::MultiLine(escape))
            } else {
                None
            }
        }
        '_' if is_thematic_break(b'_') => {
            let escape = SingleLineEscape::ThematicBreak(ThematicBreakMarker::Underscore);
            Some(EscapeKind::SingleLine(escape))
        }
        '*' => {
            if is_thematic_break(b'*') {
                let escape = SingleLineEscape::ThematicBreak(ThematicBreakMarker::Asterisk);
                Some(EscapeKind::SingleLine(escape))
            } else if is_unordered_list_marker('*') {
                let escape = SingleLineEscape::UnorderedList(UnorderedListMarker::Asterisk);
                Some(EscapeKind::SingleLine(escape))
            } else {
                None
            }
        }
        '+' if is_unordered_list_marker('+') => {
            let escape = SingleLineEscape::UnorderedList(UnorderedListMarker::Plus);
            Some(EscapeKind::SingleLine(escape))
        }
        '`' if is_fenced_code_block("```") => {
            let escape = SingleLineEscape::FencedCodeBlock(FencedCodeBlockMarker::Backtick);
            Some(EscapeKind::SingleLine(escape))
        }
        '~' if is_fenced_code_block("~~~") => {
            let escape = SingleLineEscape::FencedCodeBlock(FencedCodeBlockMarker::Tildes);
            Some(EscapeKind::SingleLine(escape))
        }
        '>' => Some(EscapeKind::SingleLine(SingleLineEscape::BlockQuote)),
        _ => None,
    }
}

/// Represents text that looks like another Markdown construct that the formatter should escape
/// in order to preserve the meaning of the input after formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EscapeKind {
    /// The formatter can be sure that this text needs to be escaped.
    SingleLine(SingleLineEscape),
    /// Context about preceding lines is required in order to know for sure that this text
    /// should be escaped
    MultiLine(MultiLineEscape),
}

impl EscapeKind {
    #[allow(dead_code)]
    pub(crate) fn marker(&self) -> char {
        match self {
            Self::SingleLine(SingleLineEscape::AtxHeader) => '#',
            Self::SingleLine(SingleLineEscape::BlockQuote) => '>',
            Self::SingleLine(SingleLineEscape::FencedCodeBlock(marker)) => (*marker).into(),
            Self::SingleLine(SingleLineEscape::ThematicBreak(marker)) => (*marker).into(),
            Self::SingleLine(SingleLineEscape::UnorderedList(marker)) => marker.into(),
            Self::MultiLine(MultiLineEscape::SetextHeader(marker)) => (*marker).into(),
        }
    }

    /// Check if we need to escape multiple characters
    pub(crate) fn multi_character_escape(&self) -> bool {
        matches!(
            self,
            EscapeKind::SingleLine(
                SingleLineEscape::FencedCodeBlock(_) | SingleLineEscape::ThematicBreak(_)
            )
        )
    }
}

/// Escapes for Markdown constructs that are defined on a single line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SingleLineEscape {
    /// Escape text that looks like an atx header `# `, `#`, `## `, `##`, ... `###### `, `######`.
    AtxHeader,
    /// Escape text that looks like a blockquote `>`.
    BlockQuote,
    /// Escape text that looks like an unordered list.
    /// ```markdown
    /// *
    /// -
    /// +
    /// ```
    UnorderedList(UnorderedListMarker),
    /// Escape text that looks like a thematic break. The text might contain whitespace.
    /// ```markdown
    /// ***
    /// * * *
    /// ___
    /// _ _ _
    /// ---
    /// - - -
    /// ```
    ThematicBreak(ThematicBreakMarker),
    /// Escape ``` or ~~~ that might look like a fenced code block.
    FencedCodeBlock(FencedCodeBlockMarker),
}

/// Escapes for Markdown constructs that are defined over multiple lines.
///
/// These escapes need context about surrounding lines in order to know for sure that they should
/// be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MultiLineEscape {
    /// Escape text that looks like a setext headers
    /// ```markdown
    /// h1
    /// =
    ///
    /// h2
    /// -
    /// ```
    SetextHeader(SetextHeaderMarker),
}

/// Types of fenced code block markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FencedCodeBlockMarker {
    /// A code fence that uses (`) character.
    Backtick,
    /// A code fence that uses (~) characters.
    Tildes,
}

impl From<FencedCodeBlockMarker> for char {
    fn from(value: FencedCodeBlockMarker) -> Self {
        match value {
            FencedCodeBlockMarker::Backtick => '`',
            FencedCodeBlockMarker::Tildes => '~',
        }
    }
}

/// Types of thematic break markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ThematicBreakMarker {
    /// A thematic break that uses `*`.
    Asterisk,
    /// A thematic break that uses `-`.
    Hyphen,
    /// A thematic break that uses `_`.
    Underscore,
}

impl From<ThematicBreakMarker> for char {
    fn from(value: ThematicBreakMarker) -> Self {
        match value {
            ThematicBreakMarker::Asterisk => '*',
            ThematicBreakMarker::Hyphen => '-',
            ThematicBreakMarker::Underscore => '_',
        }
    }
}

/// Types of setext header markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SetextHeaderMarker {
    /// A setext header that uses `=`.
    H1,
    /// A setext header that usese `-`.
    H2,
}

impl From<SetextHeaderMarker> for char {
    fn from(value: SetextHeaderMarker) -> Self {
        match value {
            SetextHeaderMarker::H1 => '=',
            SetextHeaderMarker::H2 => '-',
        }
    }
}
