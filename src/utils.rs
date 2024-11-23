use std::borrow::Cow;

use std::iter::Iterator;
use unicode_width::UnicodeWidthStr;

static SPACES: &str = "                                                                           ";
const NON_BREAKING_SPACE: char = '\u{a0}';
const SPACE: char = ' ';

/// Count the number of trailing spaces at the end of input
pub(crate) fn count_trailing_spaces(s: &str) -> usize {
    s.chars()
        .rev()
        .take_while(|c| c.is_whitespace())
        .filter(|c| matches!(*c, NON_BREAKING_SPACE | SPACE))
        .count()
}

/// Get the number of trailing spaces that you need.
pub(crate) fn get_spaces(n: usize) -> Cow<'static, str> {
    if n <= SPACES.len() {
        SPACES[..n].into()
    } else {
        " ".repeat(n).into()
    }
}

// Duplicated from the rustfmt::util module
pub(crate) fn unicode_str_width(s: &str) -> usize {
    s.width()
}

// Too clever? Is there a simpler way to do this that I'm not realizing?
pub(crate) fn is_char_esacped(c: char, prev_escape_char: bool) -> bool {
    // By applying the `xor` + `and` operation we get the following truth table.
    // This helps us determine if we're escaping an escape character (\\), or
    // if we're escaping a meaningful character liks `\]`, which esacapes the closing
    // bracket for a link label.
    //
    // `\]`  == escaped
    // `\\]` != escaped
    //
    // | prev_escape_char (A) | is_escape_char (B) | A xor B (C)  | C && B |
    // | -------------------- | ------------------ | ------------ | ------ |
    // | true                 | true               | false        | false  |
    // | false                | true               | true         | true   |
    // | true                 | false              | true         | false  |
    // | false                | false              | false        | false  |
    let is_escape_char = c == '\\';
    (prev_escape_char ^ is_escape_char) && is_escape_char
}

pub(crate) fn sequence_ends_on_escape(s: &str) -> bool {
    s.chars().fold(false, |prev, current_char| {
        is_char_esacped(current_char, prev)
    })
}

const CARRIAGE_RETURN: u8 = b'\r';
const LINE_FEED: u8 = b'\n';

pub(crate) fn count_newlines(s: &str) -> usize {
    let mut iter = s.bytes().peekable();
    let mut newline_count = 0;

    while let Some(b) = iter.next() {
        match b {
            CARRIAGE_RETURN => {
                if matches!(iter.peek(), Some(&LINE_FEED)) {
                    // Advance the iter past the CRLF(\r\n)
                    iter.next();
                }
                newline_count += 1;
            }
            LINE_FEED => {
                newline_count += 1;
            }
            _ => {}
        }
    }
    newline_count
}

struct SplitLines<'input> {
    input: &'input str,
    offset: usize,
}

impl<'input> Iterator for SplitLines<'input> {
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.input.len() {
            return None;
        }

        let mut iter = self.input[self.offset..].bytes().peekable();
        let mut new_offset = 0;

        while let Some(b) = iter.next() {
            new_offset += 1;
            match b {
                CARRIAGE_RETURN => {
                    if matches!(iter.peek(), Some(&LINE_FEED)) {
                        // Advance the iter past the CRLF(\r\n)
                        new_offset += 1;
                    }
                    break;
                }
                LINE_FEED => {
                    break;
                }
                _ => {}
            }
        }

        let end = self.offset + new_offset;
        let result = &self.input[self.offset..end].trim_end_matches(['\r', '\n']);
        self.offset = end;
        Some(result)
    }
}

pub(crate) fn split_lines(s: &str) -> impl Iterator<Item = &str> {
    SplitLines {
        input: s,
        offset: 0,
    }
}

#[test]
fn make_sure_sequence_ends_on_escape_works() {
    // Sequences that end on an unescaped backslash
    assert!(sequence_ends_on_escape("\\"));
    assert!(sequence_ends_on_escape("\\\\\\"));
    assert!(sequence_ends_on_escape("\\\\\\\\\\"));
    assert!(sequence_ends_on_escape("\\\\\\\\\\\\\\"));
    assert!(sequence_ends_on_escape("😁\\"));
    assert!(sequence_ends_on_escape("😁\\\\\\"));
    assert!(sequence_ends_on_escape("😁\\\\\\\\\\"));
    assert!(sequence_ends_on_escape("😁\\\\\\\\\\\\\\"));

    // Sequences That don't end on an unescaped backslash
    assert!(!sequence_ends_on_escape("\\\\"));
    assert!(!sequence_ends_on_escape("\\\\#"));
    assert!(!sequence_ends_on_escape("\\\\["));
    assert!(!sequence_ends_on_escape("\\\\]"));
    assert!(!sequence_ends_on_escape("\\\\}"));
    assert!(!sequence_ends_on_escape("\\\\{"));
    assert!(!sequence_ends_on_escape("\\\\\\🥳"));
}

#[test]
fn test_count_newlines() {
    assert_eq!(count_newlines("\r"), 1);
    assert_eq!(count_newlines("\n>"), 1);
    assert_eq!(count_newlines("\r\n>"), 1);
    assert_eq!(count_newlines("\r\r\n\n"), 3);
    assert_eq!(count_newlines("\r\r\r"), 3);
    assert_eq!(count_newlines("\n\n\n"), 3);
    assert_eq!(count_newlines("\r\n\r\n\r\n"), 3);
    assert_eq!(count_newlines(""), 0);
    assert_eq!(count_newlines(">"), 0);
    assert_eq!(count_newlines("*"), 0);
}

#[test]
fn test_split_lines() {
    for input in ["\r", "\r\n", "\n"] {
        let mut iter = split_lines(input);
        assert_eq!(iter.next(), Some(""));
        assert_eq!(iter.next(), None);
    }

    for input in ["\r\r\n\n", "\r\n\r\n\r\n", "\n\n\n"] {
        let mut iter = split_lines(input);
        assert_eq!(iter.next(), Some(""));
        assert_eq!(iter.next(), Some(""));
        assert_eq!(iter.next(), Some(""));
        assert_eq!(iter.next(), None);
    }
}
