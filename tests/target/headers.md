# One [some link](url)

Two
[some link](url)
followed by footnotes
[^1]
==

Three
[some link](url)
followed by footnote on the same line [^2]
==

> **Four**
> Some `nested` ~~setext~~ header
> [^2]
> ==


*
  + > Five
    > Another deeply nested setext header
    > [^2]
    > ---------


[^1]:
    one

[^2]:
    two

[^3]:
    *
      + > Six
        > Another deeply nested setext header
        > [^2] {#with-id}
        > ---------

> ```markdown
> (Seven) Header inside nested markdown
> {attr=value}
> ----
>
> [^4]:
> ```


### Eight with trailing escaped hash \# {#id3}

<!-- Nine empty {} replaced by \\ -->
\
==

<!-- Ten empty {} removed. \\ remains -->
\\
--

<!-- Eleven only empty \\ stay -->
\\
--

<!-- Twelve {\\} stay because the '}' is escaped -->
{\\}
==

<!-- Thirteen escape the {} so that the output is idempotent -->
\{\}
--

<!-- Fourteen escape the trailing `#` -->
hey #
===

<!-- keep the {\\} -->
Fifteen {\\}
--

<!-- no change -->
Sixteen
--

<!-- remove the empty {} -->
Seventeen
--

<!-- escape the first {} -->
Eighteen \{\}
--

<!-- \{\} remains the same -->
Nineteen \{\}
--

<!-- {\\} remains the same -->
Twenty {\\}
--

<!-- {} \{\} remains the same -->
Twenty one {} \{\}
--

<!-- remove final {}, and escape the second to last \{\} to keep the output idempotent -->
Twenty two {} \{\}
--

<!-- {} {a=b} remains the same -->
Twenty Three \{\} {a=b}
--
