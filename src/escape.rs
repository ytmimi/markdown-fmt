use super::formatter::FormatState;

impl<I> FormatState<'_, '_, I>
where
    I: Iterator,
{
    pub(super) fn needs_escape(
        &mut self,
        input: &str,
        is_inline_element: bool,
    ) -> Option<EscapeKind> {
        let first_char = input.chars().next()?;

        if !is_inline_element {
            if !self.last_was_softbreak {
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

    let is_atx_heading = || -> bool {
        let mut leading_marker_count = 0;
        let mut whitespace_after_header_marker = false;
        for c in input.chars() {
            if c == '#' {
                leading_marker_count += 1;
                continue;
            }

            if c.is_whitespace() {
                whitespace_after_header_marker = true;
            }

            break;
        }

        let empty_header = || input.chars().all(|c| c == '#');

        leading_marker_count <= 6 && (whitespace_after_header_marker || empty_header())
    };
    let is_setext_heading = |value: u8| input.trim_end().bytes().all(|b| b == value);
    let is_unordered_list_marker = |value: &str| input.starts_with(value);
    let is_thematic_break = |value: u8| input.bytes().all(|b| b == value || b == b' ');
    let is_fenced_code_block = |value: &str| input.starts_with(value);

    match first_char {
        '#' if is_atx_heading() => Some(EscapeKind::SingleLine(SingleLineEscape::AtxHeader)),
        '=' if is_setext_heading(b'=') => {
            Some(EscapeKind::MultiLine(MultiLineEscape::SetextHeader))
        }
        '-' => {
            if is_thematic_break(b'-') {
                Some(EscapeKind::SingleLine(SingleLineEscape::ThematicBreak))
            } else if is_unordered_list_marker("- ") {
                Some(EscapeKind::SingleLine(SingleLineEscape::UnorderedList))
            } else if is_setext_heading(b'-') {
                Some(EscapeKind::MultiLine(MultiLineEscape::SetextHeader))
            } else {
                None
            }
        }
        '_' if is_thematic_break(b'_') => {
            Some(EscapeKind::SingleLine(SingleLineEscape::ThematicBreak))
        }
        '*' => {
            if is_thematic_break(b'*') {
                Some(EscapeKind::SingleLine(SingleLineEscape::ThematicBreak))
            } else if is_unordered_list_marker("* ") {
                Some(EscapeKind::SingleLine(SingleLineEscape::UnorderedList))
            } else {
                None
            }
        }
        '+' if is_unordered_list_marker("+ ") => {
            Some(EscapeKind::SingleLine(SingleLineEscape::UnorderedList))
        }
        '`' if is_fenced_code_block("```") => {
            Some(EscapeKind::SingleLine(SingleLineEscape::FencedCodeBlock))
        }
        '~' if is_fenced_code_block("~~~") => {
            Some(EscapeKind::SingleLine(SingleLineEscape::FencedCodeBlock))
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
    UnorderedList,
    /// Escape text that looks like a thematic break. The text might contain whitespace.
    /// ```markdown
    /// ***
    /// * * *
    /// ___
    /// _ _ _
    /// ---
    /// - - -
    /// ```
    ThematicBreak,
    /// Escape ``` or ~~~ that might look like a fenced code block.
    FencedCodeBlock,
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
    SetextHeader,
}
