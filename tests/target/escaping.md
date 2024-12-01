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

<!--
    Escape the multi-line code text that looks like the delimter rows of a
    GitHub Flavored Markdown Table, so it won't be interpreted as one on future formatting runs.
-->
> * `qy|?-
>   \-|-
>   \|-
>   \|-   ` -`
>   `

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


<!-- Don't need to escape the double **. It won't be interpreted as a list -->

**
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


<!--
  escape the `]` in a and b so that we don't change the meaning of the markdown and
  to keep the output idempotent.
-->
a[ \][\^Inline](^)
b[ \][\^Reference][\^]
c[ ][^Collapsed][] <!-- not parsed as a link -->
d[ ][^Shortcut] <!-- not parsed as a link -->
e[ ]<https://Autolink.com>
f[ ]<Email@example.com>


<!-- escape `#` so that we don't treat it as a header -->
<!
\# *<!  
``

<!
\## *<!  
``

<!
\### *<!  
``

<!
\#### *<!  
``

<!
\##### *<!  
``

<!
\###### *<!  
``

<!-- doesn't need an escape because a header can only be up to h6 -->
<!
####### *<!  
``

<!-- escape '#' so that it's not treated as an empty header -->
A  
\#

B  
\##

C  
\###

D  
\####

E  
\#####

F  
\######

<!-- doesn't need an escape because a header can only be up to h6 -->
G  
#######
