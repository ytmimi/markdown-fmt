// @generated
// generated running `cargo build -F gen-tests`
// test macros are defined in tests/common/mod.rs
#![allow(missing_docs)]
mod common;

#[test]
fn pulldown_cmark_markdown_basic_usage_1() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L23-L37
    test_identical_markdown_events!(r##"with the ID {#myh1}
===================
with a class {.myclass}
------------
with a custom attribute {myattr=myvalue}
========================================
multiple! {.myclass1 myattr #myh3 otherattr=value .myclass2}
--"##,r##"with the ID {#myh1}
===================
with a class {.myclass}
------------
with a custom attribute {myattr=myvalue}
========================================
multiple! {#myh3 .myclass1 .myclass2 myattr otherattr=value}
--"##);
}

#[test]
fn pulldown_cmark_markdown_basic_usage_2() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L41-L51
    test_identical_markdown_events!(r##"# with the ID {#myh1}
## with a class {.myclass}
#### with a custom attribute {myattr=myvalue}
### multiple! {.myclass1 myattr #myh3 otherattr=value .myclass2}"##,r##"# with the ID {#myh1}
## with a class {.myclass}
#### with a custom attribute {myattr=myvalue}
### multiple! {#myh3 .myclass1 .myclass2 myattr otherattr=value}"##);
}

// use test! macro because after formatting the {#id4} is parsed as an id
#[test]
fn pulldown_cmark_markdown_basic_usage_3() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L55-L66
    test!(r##"# H1 # {#id1}
## H2 ## with ## multiple ## hashes ## {#id2}
### with trailing hash # ### {#id3}

#### non-attribute-block {#id4} ####"##,r##"# H1 {#id1}
## H2 ## with ## multiple ## hashes {#id2}
### with trailing hash \# {#id3}

#### non-attribute-block {#id4}"##);
}

#[test]
fn pulldown_cmark_markdown_basic_usage_4() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L70-L76
    test_identical_markdown_events!("# spaces {#myid1}    \n## tabs {#myid2}\t\t",r##"# spaces {#myid1}
## tabs {#myid2}"##);
}

#[test]
fn pulldown_cmark_markdown_basic_usage_5() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L80-L86
    test_identical_markdown_events!(r##"# H1 \
nextline"##);
}

#[test]
fn pulldown_cmark_markdown_basic_usage_6() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L90-L106
    test_identical_markdown_events!(r##"# H1 \
{#myid}

## H2 \
nextline {.class}

### H3 [link
](https://example.com/) {#myid3}"##);
}

#[test]
fn pulldown_cmark_markdown_basic_usage_7() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L117-L126
    test_identical_markdown_events!(r##"H1
cont
{#myid}
=="##);
}

#[test]
fn pulldown_cmark_markdown_basic_usage_8() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L130-L143
    test_identical_markdown_events!(r##"H1
{
  .class1
  .class2
}
=="##,r##"H1
{
.class1
.class2
}
=="##);
}

#[test]
fn pulldown_cmark_markdown_leading_spaces_9() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L161-L167
    test_identical_markdown_events!(r##"# without space, not recommended{#id1}
## recommended style with spaces {#id2}"##,r##"# without space, not recommended {#id1}
## recommended style with spaces {#id2}"##);
}

#[test]
fn pulldown_cmark_markdown_spaces_inside_braces_10() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L174-L182
    test_identical_markdown_events!(r##"# H1 { #id1 }
## H2 {.myclass      #id2 }
### H3 {     .myclass}"##,r##"# H1 {#id1}
## H2 {#id2 .myclass}
### H3 {.myclass}"##);
}

#[test]
fn pulldown_cmark_markdown_separators_11() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L188-L194
    test_identical_markdown_events!(r##"# H1 {#id1.class1.class2 .class3}
## H2 {.class1#id2.class2}"##);
}

#[test]
fn pulldown_cmark_markdown_unclosed_braces_12() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L201-L207
    test_identical_markdown_events!(r##"# H1 { #id1
## H2 {#id2"##);
}

#[test]
fn pulldown_cmark_markdown_unclosed_braces_13() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L211-L217
    test_identical_markdown_events!(r##"# H1 #id1 }
## H2 #id2}"##);
}

#[test]
fn pulldown_cmark_markdown_non_suffix_block_14() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L223-L229
    test_identical_markdown_events!(r##"# H1 { #id1 } foo
## H2 {#id2} <!-- hello -->"##);
}

#[test]
fn pulldown_cmark_markdown_inlines_15() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L235-L247
    test_identical_markdown_events!(r##"# *H1* { #id1 }
## **H2** {#id2}
### _H3_ {#id3}
#### ~~H4~~ {#id4}
##### [text](uri) {#id5}"##,r##"# *H1* {#id1}
## **H2** {#id2}
### _H3_ {#id3}
#### ~~H4~~ {#id4}
##### [text](uri) {#id5}"##);
}

#[test]
fn pulldown_cmark_markdown_id_16() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L257-L261
    test_identical_markdown_events!(r##"# H1 {#first #second #last}"##,r##"# H1 {#last}"##);
}

#[test]
fn pulldown_cmark_markdown_classes_17() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L267-L271
    test_identical_markdown_events!(r##"# H1 {.z .a .zz}"##);
}

#[test]
fn pulldown_cmark_markdown_classes_18() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L275-L279
    test_identical_markdown_events!(r##"# H1 {.a .a .a}"##);
}

#[test]
fn pulldown_cmark_markdown_combined_19() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L285-L291
    test_identical_markdown_events!(r##"# H1 {.myclass #myid}
## H2 {.z #m .a}"##,r##"# H1 {#myid .myclass}
## H2 {#m .z .a}"##);
}

#[test]
fn pulldown_cmark_markdown_custom_attributes_20() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L301-L307
    test_identical_markdown_events!(r##"# H1 {foo}
## H2 {#myid unknown this#is.ignored attr=value .myclass}"##,r##"# H1 {foo}
## H2 {#myid .myclass unknown this#is.ignored attr=value}"##);
}

#[test]
fn pulldown_cmark_markdown_custom_attributes_21() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L309-L313
    test_identical_markdown_events!(r##"# Header # {myattr=value other_attr}"##,r##"# Header {myattr=value other_attr}"##);
}

#[test]
fn pulldown_cmark_markdown_custom_attributes_22() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L315-L319
    test_identical_markdown_events!(r##"#### Header {#id myattr= .class1 other_attr=false}"##,r##"#### Header {#id .class1 myattr= other_attr=false}"##);
}

#[test]
fn pulldown_cmark_markdown_forbidden_characters_23() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L335-L341
    test_identical_markdown_events!(r##"# H1 {.foo{unknown}
## H2 {.foo{.bar}"##,r##"# H1 {.foo {unknown}
## H2 {.foo {.bar}"##);
}

#[test]
fn pulldown_cmark_markdown_forbidden_characters_24() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L345-L349
    test_identical_markdown_events!(r##"# H1 {.foo}bar}"##);
}

#[test]
fn pulldown_cmark_markdown_forbidden_characters_25() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L353-L357
    test_identical_markdown_events!(r##"# H1 {<i>foo</i>}"##);
}

#[test]
fn pulldown_cmark_markdown_forbidden_characters_26() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L361-L365
    test_identical_markdown_events!(r##"# H1 {.foo\}"##);
}

#[test]
fn pulldown_cmark_markdown_forbidden_characters_27() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L369-L376
    test_identical_markdown_events!(r##"H1 {.foo
.bar}
=="##);
}

// use test! macro since the strings are different
#[test]
fn pulldown_cmark_markdown_cancelling_parsing_of_attribute_blocks_28() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L385-L393
    test!(r##"H1 {} {}
=====

## H2 {} {}"##,r##"H1 \{\}
=====

## H2 \{\}"##);
}

// use test! macro since the strings are different
#[test]
fn pulldown_cmark_markdown_cancelling_parsing_of_attribute_blocks_29() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L397-L401
    test!(r##"## H2 {} ##"##,r##"## H2 \{\}"##);
}

#[test]
fn pulldown_cmark_markdown_cancelling_parsing_of_attribute_blocks_30() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L406-L418
    test_identical_markdown_events!(r##"# H1 {\}
## this is also ok \{\}

newline can be used for setext heading {
}
--"##);
}

#[test]
fn pulldown_cmark_markdown_cancelling_parsing_of_attribute_blocks_31() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L424-L432
    test_identical_markdown_events!(r##"# H1 \{.foo}
## H2 \\{.bar}
### stray backslash at the end is preserved \"##,r##"# H1 \{.foo}
## H2 \\ {.bar}
### stray backslash at the end is preserved \"##);
}

#[test]
fn pulldown_cmark_markdown_cancelling_parsing_of_attribute_blocks_32() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L455-L467
    test_identical_markdown_events!(r##"H1 \{.foo}
==
H2 \\{.bar}
--

stray backslash at the end is preserved \
--"##,r##"H1 \{.foo}
==
H2 \\ {.bar}
--

stray backslash at the end is preserved \
--"##);
}

#[test]
fn pulldown_cmark_markdown_disabled_inlines_33() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L474-L482
    test_identical_markdown_events!(r##"# H1 {#`code`}
## H2 {#foo__bar__baz}
### H3 {#foo**bar**baz}"##);
}

#[test]
fn pulldown_cmark_markdown_disabled_inlines_34() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L484-L497
    test_identical_markdown_events!(r##"H1 {#`code`}
==

H2-1 {#foo__bar__baz}
----

H2-2 {#foo**bar**baz}
--"##);
}

// use test! macro because after formatting Text events are slightly different
#[test]
fn pulldown_cmark_markdown_disabled_inlines_35() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L502-L512
    test!(r##"# H1 __{#my__id1}
## H2 **{#my**id2}
### H3 `{.code` }
#### H4 ~~{.strike~~ }"##,r##"# H1 __ {#my__id1}
## H2 ** {#my**id2}
### H3 ` {.code`}
#### H4 ~~ {.strike~~}"##);
}

#[test]
fn pulldown_cmark_markdown_disabled_inlines_36() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L514-L524
    test_identical_markdown_events!(r##"# H1__ {#my__id1}
## H2** {#my**id2}
### H3` {.code` }
#### H4~~ {.strike~~ }"##,r##"# H1__ {#my__id1}
## H2** {#my**id2}
### H3` {.code`}
#### H4~~ {.strike~~}"##);
}

#[test]
fn pulldown_cmark_markdown_disabled_inlines_37() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L526-L532
    test_identical_markdown_events!(r##"# H1__ {.foo__bar**baz}
qux**"##);
}

#[test]
fn pulldown_cmark_markdown_escapes_38() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L547-L555
    test_identical_markdown_events!(r##"# H1 {.foo#bar}
## H2 {#foo.bar}
### H3 {.a"b'c&d}"##);
}

#[test]
fn pulldown_cmark_markdown_empty_ids_and_classes_39() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L571-L577
    test_identical_markdown_events!(r##"# H1 {#}
## H2 {.}"##,r##"# H1
## H2"##);
}

#[test]
fn pulldown_cmark_markdown_empty_ids_and_classes_40() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L579-L585
    test_identical_markdown_events!(r##"# H1 {#foo #}
# H1 {.foo . . .bar}"##,r##"# H1 {#foo}
# H1 {.foo .bar}"##);
}

// use test! macro because after formatting empty attributes are escaped, which slightly changes events
#[test]
fn pulldown_cmark_markdown_empty_headers_41() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L589-L602
    test!(r##"# {}
## {}
### {\}
#### {} {}

#{}"##,r##"#
##
### {\}
#### \{\}

#{}"##);
}

// use test! macro because after formatting some empty attributes are replaced with escapes
#[test]
fn pulldown_cmark_markdown_empty_headers_42() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L604-L625
    test!(r##"{}
==

\{}
--

\
--

{\}
==

{}{}
--"##,r##"\
==

\
--

\
--

{\}
==

\{\}
--"##);
}

// use test! macro because some whitespace characters change
#[test]
fn pulldown_cmark_markdown_trailing_ascii_whitespaces_43() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L634-L648
    test!("# horizontal tab\t\n# horizontal tab\t{#ht}\n## form feed\u{c}\n## form feed\u{c}{#ff}\n### vertical tab\u{b}\n### vertical tab\u{b}{#vt}",r##"# horizontal tab
# horizontal tab {#ht}
## form feed
## form feed {#ff}
### vertical tab
### vertical tab {#vt}"##);
}

#[test]
fn pulldown_cmark_markdown_attributes_separators_44() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L661-L670
    test_identical_markdown_events!(r##"# horizontal tab (U+000A) {#ht	.myclass}
## form feed (U+000C) {#ff.myclass}

# vertical tab (U+000B) {#vt.myclass}"##,r##"# horizontal tab (U+000A) {#ht .myclass}
## form feed (U+000C) {#ff .myclass}

# vertical tab (U+000B) {#vt.myclass}"##);
}

#[test]
fn pulldown_cmark_markdown_attributes_separators_45() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/heading_attrs.txt#L674-L680
    test_identical_markdown_events!(r##"# EN SPACE (U+2002) {#en-space .myclass}
## IDEOGRAPHIC SPACE (U+3000) {#ideographic-space　.myclass}"##);
}
