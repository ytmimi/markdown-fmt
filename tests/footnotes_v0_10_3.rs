// @generated
// generated running `cargo build -F gen-tests`
// test macros are defined in tests/common/mod.rs
#![allow(missing_docs)]
mod common;

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_1() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L12-L21
    test!(r##"Lorem ipsum.[^a] [^missing]

[^a]: Cool."##,r##"Lorem ipsum.[^a] [^missing]

[^a]:
    Cool."##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_2() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L26-L39
    test_identical_markdown_events!(r##"> This is the song that never ends.\
> Yes it goes on and on my friends.[^lambchops]
>
> [^lambchops]: <https://www.youtube.com/watch?v=0U2zJOryHKQ>"##,r##"> This is the song that never ends.\
> Yes it goes on and on my friends.[^lambchops]
>
> [^lambchops]:
>     <https://www.youtube.com/watch?v=0U2zJOryHKQ>"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_3() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L46-L61
    test_identical_markdown_events!(r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
 * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
 * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
 * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)"##,r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
* [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
* [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
* [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_4() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L66-L82
    test_identical_markdown_events!(r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
    * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
    * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)"##,r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
    * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
    * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_5() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L87-L118
    test_identical_markdown_events!(r##"[^not-code] [^code] [^quote] [^not-quote] [^indented-quote]

[^not-code]:         not code

[^code]:
        code

[^quote]: > quote

[^not-quote]:
 > external quote

[^indented-quote]:
    > indented quote"##,r##"[^not-code] [^code] [^quote] [^not-quote] [^indented-quote]

[^not-code]:
    not code

[^code]:
        code

[^quote]:
    > quote

[^not-quote]:
> external quote

[^indented-quote]:
    > indented quote"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_6() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L124-L142
    test_identical_markdown_events!(r##"[^ab] [^cd]

[^ab]: a
b

[^cd]: c\
d"##,r##"[^ab] [^cd]

[^ab]:
    a
    b

[^cd]:
    c\
    d"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_7() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L147-L164
    test_identical_markdown_events!(r##"[^lorem]: If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.

[^ipsum]: If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

    I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp."##,r##"[^lorem]:
    If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.

[^ipsum]:
    If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

    I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp."##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_8() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L170-L183
    test_identical_markdown_events!(r##"[^ipsum]: How much wood would a woodchuck chuck.

If a woodchuck could chuck wood.


# Forms of entertainment that aren't childish"##,r##"[^ipsum]:
    How much wood would a woodchuck chuck.

If a woodchuck could chuck wood.


# Forms of entertainment that aren't childish"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_9() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L190-L217
    test_identical_markdown_events!(r##"Footnotes [^one] [^many].

[^one]:





    first paragraph inside footnote

[^many]: first paragraph inside footnote





    second paragraph still inside footnote"##,r##"Footnotes [^one] [^many].

[^one]:
    first paragraph inside footnote

[^many]:
    first paragraph inside footnote





    second paragraph still inside footnote"##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_10() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L222-L236
    test!(r##"> He's also really stupid. [^why]
>
> [^why]: Because your mamma!

As such, we can guarantee that the non-childish forms of entertainment are probably more entertaining to adults, since, having had a whole childhood doing the childish ones, the non-childish ones are merely the ones that haven't gotten boring yet."##,r##"> He's also really stupid. [^why]
>
> [^why]:
>     Because your mamma!

As such, we can guarantee that the non-childish forms of entertainment are probably more entertaining to adults, since, having had a whole childhood doing the childish ones, the non-childish ones are merely the ones that haven't gotten boring yet."##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_11() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L242-L280
    test!(r##"Nested footnotes are considered poor style. [^a] [^xkcd] [^indent1] [^indent2]

[^a]: This does not mean that footnotes cannot reference each other. [^b]

[^b]: This means that a footnote definition cannot be directly inside another footnote definition.
> This means that a footnote cannot be directly inside another footnote's body. [^e]
>
> [^e]: They can, however, be inside anything else.

[^xkcd]: [The other kind of nested footnote is, however, considered poor style.](https://xkcd.com/1208/)

[^indent1]: indent1

    [^indent2]: indent2"##,r##"Nested footnotes are considered poor style. [^a] [^xkcd] [^indent1] [^indent2]

[^a]:
    This does not mean that footnotes cannot reference each other. [^b]

[^b]:
    This means that a footnote definition cannot be directly inside another footnote definition.
> This means that a footnote cannot be directly inside another footnote's body. [^e]
>
> [^e]:
>     They can, however, be inside anything else.

[^xkcd]:
    [The other kind of nested footnote is, however, considered poor style.](https://xkcd.com/1208/)

[^indent1]:
    indent1

[^indent2]:
    indent2"##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_12() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L282-L292
    test!(r##"[^foo] [^bar]

[^foo]: [^bar]: 1"##,r##"[^foo] [^bar]

[^foo]:
[^bar]:
    1"##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_13() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L296-L309
    test!(r##"[^Doh] Ray Me Fa So La Te Do! [^1]

[^Doh]: I know. Wrong Doe. And it won't render right.
[^1]: Common for people practicing music."##,r##"[^Doh] Ray Me Fa So La Te Do! [^1]

[^Doh]:
    I know. Wrong Doe. And it won't render right.
[^1]:
    Common for people practicing music."##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_14() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L313-L331
    test!(r##"Lorem ipsum.[^a]

An unordered list before the footnotes:
* Ipsum
* Lorem

[^a]: Cool."##,r##"Lorem ipsum.[^a]

An unordered list before the footnotes:
* Ipsum
* Lorem

[^a]:
    Cool."##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_15() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L340-L391
    test!(r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]: * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
* [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
* [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)


Songs that simply loop are a popular way to annoy people. [^examples2]

[^examples2]: * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ) 2
    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls) 2
    - [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ) 2


Songs that simply loop are a popular way to annoy people. [^examples3]

[^examples3]: * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ) 3

    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls) 3

    * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ) 3"##,r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
    * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
* [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
* [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)


Songs that simply loop are a popular way to annoy people. [^examples2]

[^examples2]:
    * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ) 2
    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls) 2
    - [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ) 2


Songs that simply loop are a popular way to annoy people. [^examples3]

[^examples3]:
    * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ) 3

    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls) 3

    * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ) 3"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_16() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L405-L437
    test_identical_markdown_events!(r##"My [cmark-gfm][^c].

My [cmark-gfm][cmark-gfm][^c].

My [cmark-gfm][][^c].

My [cmark-gfm] [^c].

My [cmark-gfm[^c]].

[cmark-gfm]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702

[^c]: cmark-gfm is under the MIT license, so incorporating parts of its
    test suite into pulldown-cmark should be fine.


My [otherlink[^c]].

[otherlink[^c]]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702"##,r##"My [cmark-gfm][^c].

My [cmark-gfm][cmark-gfm][^c].

My [cmark-gfm][][^c].

My [cmark-gfm] [^c].

My [cmark-gfm[^c]].

[cmark-gfm]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702

[^c]:
    cmark-gfm is under the MIT license, so incorporating parts of its
    test suite into pulldown-cmark should be fine.


My [otherlink[^c]].

[otherlink[^c]]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702"##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_17() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L442-L461
    test!(r##"[^1]: footnote definition text

<!-- -->

    // indented code block
    fn main() {
        println!("hello world!");
    }"##,r##"[^1]:
    footnote definition text

<!-- -->

    // indented code block
    fn main() {
        println!("hello world!");
    }"##);
}

// using test macro becase we add extra whitespce
#[test]
fn pulldown_cmark_markdown_footnotes_18() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L466-L474
    test!(r##"[^1]: footnote definition text
[^1]\: this is a reference, rather than a definition"##,r##"[^1]:
    footnote definition text
    [^1]\: this is a reference, rather than a definition"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_19() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L481-L499
    test_identical_markdown_events!(r##"[^1]:

    | column1 | column2 |
    |---------|---------|
    | row1a   | row1b   |
    | row2a   | row2b   |"##,r##"[^1]:
    | column1 | column2 |
    | ------- | ------- |
    | row1a   | row1b   |
    | row2a   | row2b   |"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_20() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L510-L566
    test_identical_markdown_events!(r##"* First
  [^1]: test
* Second [^1] test


> first
> [^2]: test
> Second [^2] test


   First   | Second
-----------|----------
first      | second
[^3]: test | test [^3]


|    First   | Second    |
|------------|-----------|
| first      | second    |
| [^4]: test | test [^4] |

> [^5]: * test [^5]"##,r##"* First
  [^1]:
      test
* Second [^1] test


> first
> [^2]:
>     test
>     Second [^2] test


| First | Second |
| ----- | ------ |
| first | second |
[^3]:
    test | test [^3]


| First      | Second    |
| ---------- | --------- |
| first      | second    |
| [^4]: test | test [^4] |

> [^5]:
>     * test [^5]"##);
}

// using the test macro because we need to add escapes
#[test]
fn pulldown_cmark_markdown_footnotes_21() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L570-L576
    test!(r##"Test [^] link

[^]: https://rust-lang.org"##,r##"Test [\^] link

[\^]: https://rust-lang.org"##);
}

// using the test macro because we need to add escapes
#[test]
fn pulldown_cmark_markdown_footnotes_22() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L580-L614
    test!(r##"[^foo\
bar]: not a footnote definition

[baz\
quux]: https://rust-lang.org

[first
second]: https://rust-lang.org

[^third
fourth]: not a footnote definition

[baz\
quux]
[^foo\
bar]
[first
second]
[^third
fourth]"##,r##"[\^foo bar]\: not a footnote definition

[baz\ quux]: https://rust-lang.org

[first second]: https://rust-lang.org

[\^third fourth]\: not a footnote definition

[baz quux]
[\^foo bar]
[first second]
[\^third fourth]"##);
}

// using the test macro because we need to add escapes
#[test]
fn pulldown_cmark_markdown_footnotes_23() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L619-L628
    test!(r##"[^foo
]: https://rust-lang.org

[^foo
]"##,r##"[\^foo]: https://rust-lang.org

[\^foo]"##);
}

#[test]
fn pulldown_cmark_markdown_footnotes_24() {
    // https://github.com/pulldown-cmark/pulldown-cmark/tree/v0.10.3/pulldown-cmark/specs/footnotes.txt#L633-L648
    test_identical_markdown_events!(r##"footnote [^baz]
footnote [^quux]

    [^quux]: x

   [^baz]: x"##,r##"footnote [^baz]
footnote [^quux]

    [^quux]: x

[^baz]:
    x"##);
}
