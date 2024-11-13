use std::borrow::Cow;
use std::num::ParseIntError;
// Including all these spaces might be overkill, but it probably doesn't hurt.
// In practice we'll see far fewer digits in an ordered list.
//
// <https://github.github.com/gfm/#list-items> mentions that:
//
//     An ordered list marker is a sequence of 1–9 arabic digits (0-9), followed by either a .
//     character or a ) character. (The reason for the length limit is that with 10 digits we
//     start seeing integer overflows in some browsers.)
//
const LIST_INDENTATION: &str = "                    ";
const ZERO_PADDING: &str = "00000000000000000000";

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ListMarker {
    Ordered {
        zero_padding: usize,
        number: usize,
        marker: OrderedListMarker,
    },
    Unordered(UnorderedListMarker),
}

impl std::default::Default for ListMarker {
    fn default() -> Self {
        ListMarker::Unordered(UnorderedListMarker::Asterisk)
    }
}

impl ListMarker {
    // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
    #[allow(dead_code)]
    pub(super) fn increment_count(&mut self) {
        match self {
            Self::Ordered { number, .. } => {
                *number += 1;
            }
            Self::Unordered(_) => {}
        }
    }

    pub(super) fn indentation(&self) -> Cow<'static, str> {
        let indent_index = self.indentation_len();

        if indent_index <= LIST_INDENTATION.len() {
            Cow::from(&LIST_INDENTATION[..indent_index])
        } else {
            // I think it would be extreamly rare to hit his case
            Cow::from(" ".repeat(indent_index))
        }
    }

    pub(super) fn marker_char(&self) -> char {
        match self {
            Self::Ordered { marker, .. } => marker.into(),
            Self::Unordered(marker) => marker.into(),
        }
    }

    pub(super) fn zero_padding(&self) -> &'static str {
        match self {
            Self::Ordered { zero_padding, .. } => &ZERO_PADDING[..*zero_padding],
            Self::Unordered(_) => "",
        }
    }

    fn indentation_len(&self) -> usize {
        match self {
            Self::Ordered {
                zero_padding,
                number,
                ..
            } => {
                let char_len = number.checked_ilog10().unwrap_or(0) + 1;
                // + 2 to for '. '
                zero_padding + (char_len + 2) as usize
            }
            Self::Unordered(_) => 2,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.indentation_len() - 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum OrderedListMarker {
    Period,
    Parenthesis,
}

impl From<&OrderedListMarker> for char {
    fn from(value: &OrderedListMarker) -> Self {
        match value {
            OrderedListMarker::Period => '.',
            OrderedListMarker::Parenthesis => ')',
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub(super) struct InvalidMarker(char);

impl TryFrom<char> for OrderedListMarker {
    type Error = InvalidMarker;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(OrderedListMarker::Period),
            ')' => Ok(OrderedListMarker::Parenthesis),
            _ => Err(InvalidMarker(value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum UnorderedListMarker {
    Asterisk,
    Plus,
    Hyphen,
}

impl From<&UnorderedListMarker> for char {
    fn from(value: &UnorderedListMarker) -> Self {
        match value {
            UnorderedListMarker::Asterisk => '*',
            UnorderedListMarker::Plus => '+',
            UnorderedListMarker::Hyphen => '-',
        }
    }
}

impl TryFrom<char> for UnorderedListMarker {
    type Error = InvalidMarker;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(UnorderedListMarker::Asterisk),
            '+' => Ok(UnorderedListMarker::Plus),
            '-' => Ok(UnorderedListMarker::Hyphen),
            _ => Err(InvalidMarker(value)),
        }
    }
}

/// Some error occured when parsing a ListMarker from a &str
#[derive(Debug, PartialEq, Eq)]
pub(super) enum ParseListMarkerError {
    /// Did not contain the correct list markers.
    NoMarkers,
    /// Invalid char where a list marker was expected
    InvalidMarker(InvalidMarker),
    /// Failed to parse an integer for ordered lists
    ParseIntError(ParseIntError),
}

impl From<InvalidMarker> for ParseListMarkerError {
    fn from(value: InvalidMarker) -> Self {
        Self::InvalidMarker(value)
    }
}

impl From<ParseIntError> for ParseListMarkerError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl std::str::FromStr for ListMarker {
    type Err = ParseListMarkerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(|c: char| c.is_whitespace() || c == '>');
        if s.is_empty() {
            return Err(ParseListMarkerError::NoMarkers);
        }

        if let Some(c @ ('*' | '+' | '-')) = s.chars().next() {
            return Ok(ListMarker::Unordered(c.try_into()?));
        }

        let Some((offset, marker)) = s.char_indices().find(|(_, c)| matches!(c, '.' | ')')) else {
            return Err(ParseListMarkerError::NoMarkers);
        };

        let number: usize = s[..offset].parse()?;
        let zero_padding = if number != 0 {
            s[..offset].bytes().take_while(|b| *b == b'0').count()
        } else {
            0
        };

        Ok(ListMarker::Ordered {
            zero_padding,
            number,
            marker: marker.try_into()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    macro_rules! check_unordered_list {
        ($string:literal, marker=$m:ident) => {
            assert_eq!(
                ListMarker::from_str($string),
                Ok(ListMarker::Unordered(UnorderedListMarker::$m))
            );
        };
    }

    #[test]
    fn parse_unordered_lists() {
        check_unordered_list!(" *", marker = Asterisk);
        check_unordered_list!(" +", marker = Plus);
        check_unordered_list!(" -", marker = Hyphen);
        check_unordered_list!("*", marker = Asterisk);
        check_unordered_list!("+", marker = Plus);
        check_unordered_list!("-", marker = Hyphen);
        check_unordered_list!("* foo", marker = Asterisk);
        check_unordered_list!("+ foo", marker = Plus);
        check_unordered_list!("- foo", marker = Hyphen);
        check_unordered_list!("* # Bar", marker = Asterisk);
        check_unordered_list!("+ # Bar", marker = Plus);
        check_unordered_list!("- # Bar", marker = Hyphen);
    }

    macro_rules! check_ordered_list {
        ($string:literal, number=$n:literal, padding=$p:literal, marker=$m:ident) => {
            assert_eq!(
                ListMarker::from_str($string),
                Ok(ListMarker::Ordered {
                    zero_padding: $p,
                    number: $n,
                    marker: OrderedListMarker::$m
                })
            );
        };
    }

    #[test]
    fn parse_ordered_lists() {
        check_ordered_list!("1.", number = 1, padding = 0, marker = Period);
        check_ordered_list!("1)", number = 1, padding = 0, marker = Parenthesis);
        check_ordered_list!("20.", number = 20, padding = 0, marker = Period);
        check_ordered_list!("20)", number = 20, padding = 0, marker = Parenthesis);
        check_ordered_list!("003.", number = 3, padding = 2, marker = Period);
        check_ordered_list!("003)", number = 3, padding = 2, marker = Parenthesis);
    }
}
