use super::formatter::FormatState;

impl<I> FormatState<'_, '_, I>
where
    I: Iterator,
{
    pub(super) fn needs_escape(&mut self, input: &str, is_inline_element: bool) -> bool {
        let Some(first_char) = input.chars().next() else {
            return false;
        };

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
                return false;
            }

            // Don't interpret the `:` as a definition list definition or blockquote
            if first_char == ':' || first_char == '>' {
                return true;
            }

            if input.len() <= 2 {
                return false;
            }
        }

        needs_escape(input)
    }
}

pub(crate) fn needs_escape(input: &str) -> bool {
    let Some(first_char) = input.chars().next() else {
        return false;
    };

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

        leading_marker_count <= 6 && whitespace_after_header_marker
    };
    let is_setext_heading = |value: u8| input.trim_end().bytes().all(|b| b == value);
    let is_unordered_list_marker = |value: &str| input.starts_with(value);
    let is_thematic_break = |value: u8| input.bytes().all(|b| b == value || b == b' ');
    let is_fenced_code_block = |value: &str| input.starts_with(value);

    match first_char {
        '#' => is_atx_heading(),
        '=' => is_setext_heading(b'='),
        '-' => is_unordered_list_marker("- ") || is_setext_heading(b'-') || is_thematic_break(b'-'),
        '_' => is_thematic_break(b'_'),
        '*' => is_unordered_list_marker("* ") || is_thematic_break(b'*'),
        '+' => is_unordered_list_marker("+ "),
        '`' => is_fenced_code_block("```"),
        '~' => is_fenced_code_block("~~~"),
        '>' | ':' => true,
        _ => false,
    }
}
