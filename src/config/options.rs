//! Configuration options

use crate::list::UnorderedListMarker;
use std::str::FromStr;

/// Normalize all unordered lists to use the same marker
///
/// See [FormatBuilder::unordered_list_marker](crate::FormatBuilder::unordered_list_marker)
/// for details on how to configure unordered list markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnorderedListMarkerConfig {
    /// All unordered lists will use `*` as their marker
    Asterisk,
    /// All unordered lists will use `+` as their marker
    Plus,
    /// All unordered lists will use `-` as their marker
    Hyphen,
}

impl FromStr for UnorderedListMarkerConfig {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(UnorderedListMarkerConfig::Asterisk),
            "+" => Ok(UnorderedListMarkerConfig::Plus),
            "-" => Ok(UnorderedListMarkerConfig::Hyphen),
            _ => Err(format!(
                "{s} is not a valid list marker. select one of *, +, or -"
            )),
        }
    }
}

impl From<UnorderedListMarkerConfig> for UnorderedListMarker {
    fn from(value: UnorderedListMarkerConfig) -> Self {
        match value {
            UnorderedListMarkerConfig::Asterisk => Self::Asterisk,
            UnorderedListMarkerConfig::Plus => Self::Plus,
            UnorderedListMarkerConfig::Hyphen => Self::Hyphen,
        }
    }
}
