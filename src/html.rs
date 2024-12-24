pub(crate) static HTML_BLOCK_TAG: &[&str] = &[
    "article",
    "header",
    "aside",
    "hgroup",
    "blockquote",
    "hr",
    "iframe",
    "body",
    "li",
    "map",
    "button",
    "object",
    "canvas",
    "ol",
    "caption",
    "output",
    "col",
    "p",
    "colgroup",
    "pre",
    "dd",
    "progress",
    "div",
    "section",
    "dl",
    "table",
    "td",
    "dt",
    "tbody",
    "embed",
    "textarea",
    "fieldset",
    "tfoot",
    "figcaption",
    "th",
    "figure",
    "thead",
    "footer",
    "tr",
    "form",
    "ul",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "video",
    "script",
    "style",
];

/// Check if the given string starts with a well known HTML block tag name
pub(crate) fn starts_with_html_block_identifier(s: &str) -> bool {
    if s.trim().is_empty() || !s.is_ascii() {
        return false;
    }

    // Combines HTML block condition 1 and 6
    let is_html_block_identifier = |value: &str| {
        HTML_BLOCK_TAG
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(value))
    };

    // line begins with the string <!--
    let html_block_condition_2 = |value: &str| value.starts_with("!--");

    // line begins with the string <?.
    let html_block_condition_3 = |value: &str| value.starts_with('?');

    // line begins with the string <! followed by an ASCII letter.
    let html_block_condition_4 = |value: &str| {
        value
            .strip_prefix('!')
            .is_some_and(|t| t.starts_with(|c: char| c.is_ascii_alphabetic()))
    };

    let maybe_html = s.split_whitespace().next().unwrap_or(s);
    is_html_block_identifier(maybe_html)
        || html_block_condition_3(maybe_html)
        || html_block_condition_2(maybe_html)
        || html_block_condition_4(maybe_html)
}
