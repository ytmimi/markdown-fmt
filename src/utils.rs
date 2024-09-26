use unicode_width::UnicodeWidthStr;

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
