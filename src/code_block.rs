use std::borrow::Cow;
use std::fmt::Write;

// TODO(ytmimi) support tab as an indent
pub(crate) static INDENTED_CODE_BLOCK_INDENTATION: &str = "    ";
static TILDE_FENCE: &str = "~~~~~~~~~~~~~~~~~~~~";
static BACKTICK_FENCE: &str = "````````````````````";

#[derive(Debug, PartialEq)]
pub(crate) struct Fence<'i> {
    /// Either a "~" or a "`" character
    marker: FenceMarker,
    /// The number of "~" or "`" character to output.
    len: usize,
    /// The info string pulldown_cmark got from parsing this code fence
    info_string: &'i str,
}

impl<'i> Fence<'i> {
    pub(crate) fn marker_char(&self) -> char {
        self.marker.as_char()
    }
    pub(crate) fn marker(&self) -> Cow<'_, str> {
        let marker_str = match self.marker {
            FenceMarker::Backtick => BACKTICK_FENCE,
            FenceMarker::Tilde => TILDE_FENCE,
        };

        if self.len <= marker_str.len() {
            Cow::Borrowed(&marker_str[..self.len])
        } else {
            Cow::Owned(self.marker.as_char().to_string().repeat(self.len))
        }
    }

    pub(crate) fn info_string(&self) -> &str {
        self.info_string
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum FenceMarker {
    /// Code fence uses "`"
    Backtick,
    /// Code fence uses "~"
    Tilde,
}

impl FenceMarker {
    fn as_char(&self) -> char {
        match self {
            Self::Backtick => '`',
            Self::Tilde => '~',
        }
    }
}

impl From<char> for FenceMarker {
    fn from(value: char) -> Self {
        match value {
            '`' => FenceMarker::Backtick,
            '~' => FenceMarker::Tilde,
            c => panic!("{c} is not a valid code fence marker"),
        }
    }
}

impl<'i> Fence<'i> {
    pub(crate) fn new(marker: char, len: usize, info_string: &'i str) -> Self {
        Self {
            marker: marker.into(),
            len,
            info_string,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CodeBlockKind<'i> {
    Indented,
    Fenced(Fence<'i>),
}

impl<'i> CodeBlockKind<'i> {
    pub(crate) fn new(fence: Option<Fence<'i>>) -> Self {
        match fence {
            None => Self::Indented,
            Some(fence) => Self::Fenced(fence),
        }
    }
}

/// A buffer where we write footnote definition text
#[derive(Debug, PartialEq)]
pub(crate) struct CodeBlock<'i> {
    buffer: String,
    kind: CodeBlockKind<'i>,
}

impl<'i> Write for CodeBlock<'i> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match &mut self.kind {
            CodeBlockKind::Fenced(Fence {
                marker,
                ref mut len,
                ..
            }) if self.buffer.is_empty() || self.buffer.ends_with('\n') => {
                let count = s
                    .trim_start()
                    .chars()
                    .take_while(|c| *c == marker.as_char())
                    .count();
                if count >= *len {
                    // We have to be update the length to be at least one more so that future
                    // formatting runs don't
                    *len = count + 1;
                }
            }
            _ => {}
        }
        self.buffer.push_str(s);
        Ok(())
    }
}

impl<'i> CodeBlock<'i> {
    pub(super) fn new(capacity: usize, kind: CodeBlockKind<'i>) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            kind,
        }
    }

    /// Check if the internal buffer is empty
    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Consume Self and return the formatted buffer
    pub(super) fn into_parts(self) -> (String, CodeBlockKind<'i>) {
        (self.buffer, self.kind)
    }
}
