// @generated
// generated running `cargo build -F gen-tests`
// test macros are defined in tests/common/mod.rs
#![allow(missing_docs)]
mod common;

#[test]
fn pulldown_cmark_markdown_metadata_blocks_1() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L18-L24
    test_identical_markdown_events!(r##"---
title: example
another_field: 0
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_2() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L27-L35
    test_identical_markdown_events!(r##"---
title: example
another_field: 0"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_3() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L38-L44
    test_identical_markdown_events!(r##"---
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_4() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L47-L57
    test_identical_markdown_events!(r##"---

title: example
another_field: 0
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_5() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L60-L70
    test_identical_markdown_events!(r##"My paragraph here.
---
title: example
another_field: 0
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_6() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L73-L82
    test_identical_markdown_events!(r##"My paragraph here.

---
title: example
another_field: 0
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_7() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L85-L99
    test_identical_markdown_events!("---    \ntitle: example\nanother_field: 0\n---\n\n--- -\ntitle: example\nanother_field: 0\n---",r##"---
title: example
another_field: 0
---

--- -
title: example
another_field: 0
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_8() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L102-L117
    test_identical_markdown_events!("---\ntitle: example\nanother_field: 0\n---        \n\n---\ntitle: example\nanother_field: 0\n---a",r##"---
title: example
another_field: 0
---

---
title: example
another_field: 0
---a"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_9() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L121-L127
    test_identical_markdown_events!(r##"---
title: example
another_field: 0
..."##,r##"---
title: example
another_field: 0
---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_10() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L132-L138
    test_identical_markdown_events!(r##"+++
title: example
another_field: 0
+++"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_11() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L142-L151
    test_identical_markdown_events!(r##"    ---
    Things
    ---"##);
}

#[test]
fn pulldown_cmark_markdown_metadata_blocks_12() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/metadata_blocks.txt#L155-L161
    test_identical_markdown_events!(r##"---
- Item 1
- Item 2
---"##);
}
