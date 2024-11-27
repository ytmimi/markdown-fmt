<!-- Don't interpret as inline HTML -->

> *<\!fJ<\!fJ`
> TT


<!-- Don't interpret as a table without a leading `|` -->

> 6|
> \-|

<!-- Escape `|-|` so it's not interpreted as a table -->
- |\!
  \|-|

<!-- Escape the `|-` so it's not interpreted as a table -->
- -|
  \|-

<!-- Escape any `|` chars inside a table -->
| `6  |
| --- |
| [\| |

<!-- Escape '-|' even when there are a lot of trailing spaces -->
[|        
\-|


<!-- Don't interpret this as a link reference definition  -->

[.]\:[:]

<!-- space hard break followed by paragraph with single `-` -->
<  
\-

<!-- Don't interpret the '```' as the start of a fenced code block -->
--
\```>

<!-- Don't interpret the '```' as the start of a fenced code block -->
--
\~~~>

<!-- Don't interpret the `--` as a setext header -->
* -+
  \--
  *-*>

<!-- Don't interpret the `==` as a setext header -->
* -+
  \==
  *-*>

<!-- Don't interpret `* ` as a list, which prevents (```) from being interpreted as a fenced code block-->
&
\* ```

<!-- Don't interpret `- ` as a list, which prevents (```) from being interpreted as a fenced code block-->
&
\- ```

<!-- Don't interpret `+ ` as a list, which prevents (```) from being interpreted as a fenced code block-->
&
\+ ```

<!-- Tight list that starts with the text `\\` and a soft break.
     Escape the `\` so it's not considered a hard break on future runs
-->
* \\
  ~

<!-- This is already escaped. Don't add any more escapes -->
[
\\[]


<!-- Don't interpret the `>` as part of the blockquote -->

> 2
> \>

<!-- Don't start a new blockquote -->

--
\>-

<!-- escape the '<p' so it's not interpreted as an HTML block -->
<
\<p  
!

<!-- Escape the escape so that we don't escape the closing `]`on the next formatting run -->
[\\]: ]


<!-- Escape the `*` so that we continue to parse this as a definition list.
     This is initially parsed as a definition list because of the trailing form-feed
-->
*

\*

:


<!-- escape what looks like rule -->
[.]: a
\***

[.]: b
\---

[.]: c
\___


<!-- Escape the `^` in the link and the unescaped escape in the label -->
[\\][\^]
