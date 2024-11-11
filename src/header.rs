use crate::utils::sequence_ends_on_escape;
use pulldown_cmark::{CowStr, HeadingLevel, Tag};
use std::borrow::Cow;
use std::fmt::Write;

/// A buffer where we write the content of markdown headers
#[derive(Debug, PartialEq)]
pub(super) struct Header<'i> {
    buffer: String,
    indentation: Vec<Cow<'static, str>>,
    kind: HeaderKind<'i>,
    attrs_on_own_line: bool,
    id: Option<CowStr<'i>>,
    classes: Vec<CowStr<'i>>,
    attrs: Vec<(CowStr<'i>, Option<CowStr<'i>>)>,
}

impl Write for Header<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl<'i> Header<'i> {
    pub(super) fn new(
        indentation: Vec<Cow<'static, str>>,
        full_header: &'i str,
        tag: Tag<'i>,
    ) -> Self {
        let Tag::Heading {
            level,
            id,
            classes,
            attrs,
        } = tag
        else {
            panic!("Tried to initialize a `Header` without a `Tag::Heading`")
        };

        let header_marker =
            if full_header.contains(['\r', '\n']) && full_header.ends_with(['=', '-']) {
                // support for alternative syntax for H1 and H2
                // <https://www.markdownguide.org/basic-syntax/#alternate-syntax>
                full_header
                    .lines()
                    .last()
                    .and_then(|maybe_last_line| {
                        // The `lines` iterator won't split on `\r` so we'll do that ourselves.
                        maybe_last_line.split('\r').next_back()
                    })
                    .map(|marker| {
                        marker
                            .trim_start_matches(|c: char| c.is_whitespace() || c == '>')
                            .trim_end()
                    })
            } else {
                None
            };

        let attrs_on_own_line = full_header
            .lines()
            .rev()
            .map(|l| l.trim_start_matches(|c: char| c.is_whitespace() || c == '>'))
            .any(|l| l.starts_with('{'));

        let kind = match (level, header_marker) {
            (HeadingLevel::H1, Some(marker)) => HeaderKind::SetextH1(marker),
            (HeadingLevel::H2, Some(marker)) => HeaderKind::SetextH2(marker),
            _ => HeaderKind::Atx(level),
        };

        Self {
            buffer: String::with_capacity(full_header.len() * 2),
            indentation,
            kind,
            attrs_on_own_line,
            id,
            classes,
            attrs,
        }
    }

    /// Check if the internal buffer is empty
    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Determine what kind of markdown header this represents
    pub(super) fn kind(&self) -> HeaderKind<'i> {
        self.kind.clone()
    }

    /// Does this header have attributes like {#id .class key=value}
    fn has_attributes(&self) -> bool {
        self.id.is_some() || !self.classes.is_empty() || !self.attrs.is_empty()
    }

    /// Consume `self` and return the buffer along with any indentaion that we took
    /// when creating `Self`.
    pub(super) fn into_parts(
        mut self,
    ) -> Result<(String, Vec<Cow<'static, str>>), std::fmt::Error> {
        if matches!(self.kind(), HeaderKind::Atx(_)) && self.buffer.trim_end().ends_with('#') {
            if self.has_attributes() {
                // Make sure we properly escape trailing `#` at the end of the header.
                // Otherwise our formatting might not be idempotent
                self.escape_trailing_hashtag()
            } else {
                // Similarly, we need to remove trailing `#` so that the output is idempotent
                self.remove_trailing_hashtags()
            }
        }

        self.escape_trailing_empty_attribute_brackets();
        self.write_header_attributes()?;
        self.write_setext_header()?;
        Ok((self.buffer, self.indentation))
    }

    fn remove_trailing_hashtags(&mut self) {
        let snippet = self.buffer.trim_end().trim_end_matches('#');
        let is_escaped = sequence_ends_on_escape(snippet);
        let no_spaces = !snippet.ends_with(char::is_whitespace);
        if is_escaped || (no_spaces && !snippet.is_empty()) {
            return;
        }

        while self
            .buffer
            .ends_with(|c: char| c.is_whitespace() || c == '#')
        {
            self.buffer.pop();
        }
    }

    // Note: Should only call this after we determine that we need to escape trailing hashtags
    fn escape_trailing_hashtag(&mut self) {
        // Get rid of any trailing whitespace after the `#`
        while self.buffer.ends_with(char::is_whitespace) {
            self.buffer.pop();
        }

        let (idx, byte_len) = self
            .buffer
            .char_indices()
            .rev()
            .find_map(|(idx, c)| {
                if c != '#' {
                    Some((idx, c.len_utf8()))
                } else {
                    None
                }
            })
            .unwrap_or((0, 0));

        let first_hashtag_byte_offset = idx + byte_len;
        let ends_with_escape = sequence_ends_on_escape(&self.buffer[..first_hashtag_byte_offset]);

        if ends_with_escape {
            // Already escaped. Nothing for us to do
            return;
        }

        let hashtag_count = self.buffer[first_hashtag_byte_offset..].chars().count();
        while self.buffer.ends_with('#') {
            self.buffer.pop();
        }
        self.buffer.push('\\');
        for _ in 0..hashtag_count {
            self.buffer.push('#');
        }
    }

    fn escape_trailing_empty_attribute_brackets(&mut self) {
        // Get rid of any trailing whitespace after the `{}`
        while self.buffer.ends_with(char::is_whitespace) {
            self.buffer.pop();
        }

        // Find out where the last line starts. Attributes can't contain any newlines
        let last_line_start_index = if self.is_setext_header() {
            self.buffer
                .rfind(['\r', '\n'])
                .map(|i| i + '\n'.len_utf8())
                .unwrap_or(0)
        } else {
            0
        };

        let mut iter = self.buffer[last_line_start_index..]
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .take(2);

        let has_empty_attr_brackets =
            matches!(iter.next(), Some('}')) && matches!(iter.next(), Some('{'));

        if !has_empty_attr_brackets {
            // Doesn't end with an empty set of {}. Nothing more to do here.
            return;
        }

        let open_bracket_index = self
            .buffer
            .rfind('{')
            .expect("We just checked that there is an open bracket");
        let ends_with_escape = sequence_ends_on_escape(&self.buffer[..open_bracket_index]);

        while !self.buffer.ends_with('{') {
            self.buffer.pop();
        }

        // Remove the '{'
        self.buffer.pop();

        if ends_with_escape {
            self.buffer.push_str("{\\}");
        } else {
            self.buffer.push_str("\\{\\}");
        }
    }

    /// Is `Self` formatting a setext header
    pub(super) fn is_setext_header(&self) -> bool {
        matches!(
            &self.kind,
            HeaderKind::SetextH1(_) | HeaderKind::SetextH2(_)
        )
    }

    /// Write the final marker line for setext headers
    fn write_setext_header(&mut self) -> std::fmt::Result {
        match self.kind {
            HeaderKind::SetextH1(s) | HeaderKind::SetextH2(s) => {
                if self.buffer.is_empty() {
                    // Escape the header so that future formatting runs don't parse the empty
                    // setext header as a paragraph
                    self.buffer.push('\\');
                }

                writeln!(self.buffer)?;
                writeln!(self.buffer, "{s}")?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Write out formatted custom header attributes
    fn write_header_attributes(&mut self) -> std::fmt::Result {
        if self.id.is_none() && self.classes.is_empty() && self.attrs.is_empty() {
            // Prevents idempotence issue for this case: `## H2 {} ##`,
            // which should foramt as `## H2`
            let mut iter = self
                .buffer
                .bytes()
                .rev()
                .filter(|b| !b.is_ascii_whitespace() && *b == b'\n')
                .take(2);
            if matches!(iter.next(), Some(b'}')) && matches!(iter.next(), Some(b'{')) {
                while self
                    .buffer
                    .ends_with(|c: char| (c.is_whitespace() && c != '\n') || matches!(c, '{' | '}'))
                {
                    self.buffer.pop();
                }
            }
            return Ok(());
        }

        let classes = std::mem::take(&mut self.classes);
        let attrs = std::mem::take(&mut self.attrs);
        let id = self.id.take();

        if self.attrs_on_own_line {
            if !self.buffer.ends_with('\n') {
                writeln!(self.buffer)?;
            }
        } else if !self.buffer.is_empty() && !sequence_ends_on_escape(&self.buffer) {
            write!(self.buffer, " ")?;
        }

        write!(self.buffer, "{{")?;

        if let Some(id) = id.as_ref() {
            write!(self.buffer, "#{id}")?;
        }

        let empty_classes = classes.is_empty();
        if !classes.is_empty() {
            rewirte_header_classes(&mut self.buffer, classes, id.is_none())?;
        }

        if !attrs.is_empty() {
            rewrite_custom_attrs(&mut self.buffer, attrs, id.is_none() || empty_classes)?;
        }

        write!(self.buffer, "}}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum HeaderKind<'i> {
    /// ATX headers like `#`, `##`, `###`, etc.
    Atx(HeadingLevel),
    /// Setext header underlined with `==`
    SetextH1(&'i str),
    /// Setext header underlined with `--`
    SetextH2(&'i str),
}

/// Rewrite custom header classes
///
/// ```markdown
/// # h1 {.class1, .class2}
/// ```
fn rewirte_header_classes<W: Write, T: AsRef<str>>(
    writer: &mut W,
    classes: Vec<T>,
    trim_start: bool,
) -> std::fmt::Result {
    for (idx, class) in classes.iter().enumerate() {
        if idx == 0 && trim_start {
            write!(writer, ".{}", class.as_ref())?;
        } else {
            write!(writer, " .{}", class.as_ref())?;
        }
    }
    Ok(())
}

/// Rewrite custom header attributes
///
/// ```markdown
/// # h1 {attr1, attr2=value}
/// ```
fn rewrite_custom_attrs<W: Write, T: AsRef<str>>(
    writer: &mut W,
    attrs: Vec<(T, Option<T>)>,
    trim_start: bool,
) -> std::fmt::Result {
    for (idx, (rhs, lhs)) in attrs.iter().enumerate() {
        match (rhs.as_ref(), lhs, idx, trim_start) {
            (r, Some(l), 0, true) => write!(writer, "{r}={}", l.as_ref())?,
            (r, Some(l), _, false | true) => write!(writer, " {r}={}", l.as_ref())?,
            (r, None, 0, true) => write!(writer, "{r}")?,
            (r, None, _, false | true) => write!(writer, " {r}")?,
        }
    }
    Ok(())
}
