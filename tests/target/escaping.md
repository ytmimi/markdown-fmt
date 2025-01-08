<!-- Don't interpret as inline HTML -->

> *<\!fJ<\!fJ`
> TT


<!-- Don't interpret as a table without a leading `|` -->

> a|
> \-|

> b|
> \-:|

* c|
  \:-:|

* d|
  \:-|


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
    GitHub Flavored Markdown Table, by adding extra indentation.
    This ensures it won't be interpreted as a table on future formatting runs.
-->
> * `qy|?-
>       -|-
>       |-
>       |-   ` -`
>   `

<!-- Don't interpret this as a link reference definition  -->

[.]\:[:]

[.]: /url
[:]: /url

<!-- space hard break followed by paragraph with single `-` -->
<  
\-

<!-- Don't interpret the '```' as the start of a fenced code block -->
--
\`\`\`>

<!-- Don't interpret the '```' as the start of a fenced code block -->
--
\~\~\~>

<!-- Don't interpret the `--` as a setext header -->
* -+
  \--
  *-*>

<!-- Don't interpret the `==` as a setext header -->
* -+
  \==
  *-*>


<!-- Setext Heading with Hardbreak -->

A  
\-
-

B  
\-
=

C  
\+
-

D  
\+
=

E  
\>
-

F  
\>
=

G  
\`\`\`
-

H  
\~\~\~
=

I  
\-\-\-
-

J  
\#
=

K  
\:
-

L  
\:
=

M\
\-
-

N\
\-
=

O\
\+
-

P\
\+
=

Q\
\>
-

R\
\>
=

S\
\`\`\`
-

T\
\~\~\~
=

U\
\-\-\-
-

V\
\#
=

W\
\:
-

X\
\:
=

<!-- Setext Heading with Softbreak -->

AA
\-
-

BB
\-
=

CC
\+
-

DD
\+
=

EE
\>
-

FF
\>
=

GG
\`\`\`
-

HH
\~\~\~
=

II
\-\-\-
-

JJ
\#
=

KK
\:
-

LL
\:
=


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

*[
\<p ~  
1


<!--
    escape HTML block condition 2
    "line begins with the string <!--"
-->
<  
\<!--o


<!--
    escape HTML block condition 3
    "line begins with the string <?"
 -->

?  
\<?a


<!--
    escape HTML block condition 4
    "line begins with the string <! followed by an ASCII letter"
-->
~  
\<!Tz


<!--
    escape HTML block condition 5
    "line begins with the string <![CDATA["
-->
5  
\<![CDATA[


<!-- Escape the escape so that we don't escape the closing `]`on the next formatting run -->
[\\]: ]


<!-- Escape the `*` so that we continue to parse this as a definition list.
     This is initially parsed as a definition list because of the trailing form-feed
-->
*

\*

:


<!-- Don't need to escape the double **. It won't be interpreted as a list -->

**
:


<!-- Escape the `^` in the link label -->
[\^][a]

[a]: url

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

[\^]: /reference

<!--
  Make sure we escape the `]` so that we don't interpret the first `[^k]` as a link on future runs.
  It's originally parsed as text
-->
][^k\][\^k][Z]

[Z]: /url

<!--
  Place an extra space between the end of the list and the start of the definition list.
  That helps to keep the output idempotent and won't change the semantics.
  input found when fuzzing.
-->

[
* (

\:
:


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

<!-- Don't need to escape because "```@``" can't be a code fence because backticks aren't allowed in the info string -->

`
```@`` 
`

<!-- The info string can contain a `~` so escape all `~` if we think this is a ~ code fence -->
=
\~\~\~ \~

<!-- We don't need to worry about escaping when the code is on a single line -->
` ```@``  `


<!-- Don't escape multi-line-code if we can help it. Instead preserve leading spaces -->

> `start of code
>     ~~~ not a code fence
> end of clode`

> `start of code
>     ``` not a code fence
> end of clode`

`start of code
    ---
end of clode`

`start of code
    ***
end of clode`

* `` start of code
      ***
  end of clode ``

+ `start of code
      >
  end of clode `

- ` start of code
      ~~~
  end of clode`

<!-- More code spans that don't need to be escaped. -->
`
```z`

<!-- Definition list title escapes Hard break -->

definition-list-A  
\-
:

definition-list-B  
\+
:

definition-list-C  
\>
:

definition-list-D  
\`\`\`
:

definition-list-E  
\~\~\~
:

definition-list-F  
\-\-\-
:

definition-list-G  
\#
:

definition-list-H  
\:
:

definition-list-I\
\-
:

definition-list-J\
\-
:

definition-list-K\
\+
:

definition-list-L\
\>
:

definition-list-M\
\`\`\`
:

definition-list-N\
\~\~\~
:

definition-list-O\
\-\-\-
:

definition-list-P\
\#
:

definition-list-Q\
\:
:

<!-- Definition list title with Softbreak -->

definition-list-R
\-
:

definition-list-S
\+
:

definition-list-T
\>
:

definition-list-U
\`\`\`
:

definition-list-V
\~\~\~
:

definition-list-W
\-\-\-
:

definition-list-X
\#
:

definition-list-Y
\:
:

<!--
    Text that looks like it should be a code block is parsed as a paragraph if it follows
    a link reference definition. We need to be mindful of escaping these lines.
-->

[A]: url 'title'
\>  <

[AA]: url 'title'
\>  <

[AAA]: url 'title'
\>  <

[A#(1)]: url 'title'
\#

[A##(2)]: url 'title'
\##

[A###(3)]: url 'title'
\###

[A####(4)]: url 'title'
\####

[A#####(5)]: url 'title'
\#####

[A######(6)]: url 'title'
\######

[A#######(7)]: url 'title'
#######

[A:]: url 'title'
:

[A```]: url 'title'
\`\`\`

[A~~~]: url 'title'
\~\~\~

[A*]: url 'title'
\*

[A+]: url 'title'
\+

[A-]: url 'title'
\-

[A***]: url 'title'
\*\*\*

[A___]: url 'title'
\_\_\_

[A---]: url 'title'
\-\-\-

[B]: url
\>  <

[BB]: url
\>  <

[B#(1)]: url
\#

[B##(2)]: url
\##

[B###(3)]: url
\###

[B####(4)]: url
\####

[B#####(5)]: url
\#####

[B######(6)]: url
\######

[B#######(7)]: url
#######

[B:]: url
:

[B```]: url
\`\`\`

[B~~~]: url
\~\~\~

[B*]: url
\*

[B+]: url
\+

[B-]: url
\-

[B***]: url
\*\*\*

[B___]: url
\_\_\_

[B---]: url
\-\-\-

> [C]: url 'title'
> \>  <

> [CC]: url 'title'
> \>  <

> [CCC]: url 'title'
> \>  <

> [C#(1)]: url 'title'
> \#

> [C##(2)]: url 'title'
> \##

> [C###(3)]: url 'title'
> \###

> [C####(4)]: url 'title'
> \####

> [C#####(5)]: url 'title'
> \#####

> [C######(6)]: url 'title'
> \######

> [C#######(7)]: url 'title'
> #######

> [C:]: url 'title'
> :

> [C```]: url 'title'
> \`\`\`

> [C~~~]: url 'title'
> \~\~\~

> [C*]: url 'title'
> \*

> [C+]: url 'title'
> \+

> [C-]: url 'title'
> \-

> [C***]: url 'title'
> \*\*\*

> [C___]: url 'title'
> \_\_\_

> [C---]: url 'title'
> \-\-\-

> [D]: url
> \>  <

> [DD]: url
> \>  <

> [D#(1)]: url
> \#

> [D##(2)]: url
> \##

> [D###(3)]: url
> \###

> [D####(4)]: url
> \####

> [D#####(5)]: url
> \#####

> [D######(6)]: url
> \######

> [D#######(7)]: url
> #######

> [D:]: url
> :

> [D```]: url
> \`\`\`

> [D~~~]: url
> \~\~\~

> [D*]: url
> \*

> [D+]: url
> \+

> [D-]: url
> \-

> [D***]: url
> \*\*\*

> [D___]: url
> \_\_\_

> [D---]: url
> \-\-\-

* [E]: url 'title'
  \>  <

* [EE]: url 'title'
  \>  <

* [EEE]: url 'title'
  \>  <

* [E#(1)]: url 'title'
  \#

* [E##(2)]: url 'title'
  \##

* [E###(3)]: url 'title'
  \###

* [E####(4)]: url 'title'
  \####

* [E#####(5)]: url 'title'
  \#####

* [E######(6)]: url 'title'
  \######

* [E#######(7)]: url 'title'
  #######

* [E:]: url 'title'
  :

* [E```]: url 'title'
  \`\`\`

* [E~~~]: url 'title'
  \~\~\~

* [E*]: url 'title'
  \*

* [E+]: url 'title'
  \+

* [E-]: url 'title'
  \-

* [E***]: url 'title'
  \*\*\*

* [E___]: url 'title'
  \_\_\_

* [E---]: url 'title'
  \-\-\-

* [F]: url
  \>  <

* [FF]: url
  \>  <

* [F#(1)]: url
  \#

* [F##(2)]: url
  \##

* [F###(3)]: url
  \###

* [F####(4)]: url
  \####

* [F#####(5)]: url
  \#####

* [F######(6)]: url
  \######

* [F#######(7)]: url
  #######

* [F:]: url
  :

* [F```]: url
  \`\`\`

* [F~~~]: url
  \~\~\~

* [F*]: url
  \*

* [F+]: url
  \+

* [F-]: url
  \-

* [F***]: url
  \*\*\*

* [F___]: url
  \_\_\_

* [F---]: url
  \-\-\-

* > [G]: url 'title'
  > \>  <

* > [GG]: url 'title'
  > \>  <

* > [GGG]: url 'title'
  > \>  <

* > [G#(1)]: url 'title'
  > \#

* > [G##(2)]: url 'title'
  > \##

* > [G###(3)]: url 'title'
  > \###

* > [G####(4)]: url 'title'
  > \####

* > [G#####(5)]: url 'title'
  > \#####

* > [G######(6)]: url 'title'
  > \######

* > [G#######(7)]: url 'title'
  > #######

* > [G:]: url 'title'
  > :

* > [G```]: url 'title'
  > \`\`\`

* > [G~~~]: url 'title'
  > \~\~\~

* > [G*]: url 'title'
  > \*

* > [G+]: url 'title'
  > \+

* > [G-]: url 'title'
  > \-

* > [G***]: url 'title'
  > \*\*\*

* > [G___]: url 'title'
  > \_\_\_

* > [G---]: url 'title'
  > \-\-\-

* > [H]: url
  > \>  <

* > [HH]: url
  > \>  <

* > [H#(1)]: url
  > \#

* > [H##(2)]: url
  > \##

* > [H###(3)]: url
  > \###

* > [H####(4)]: url
  > \####

* > [H#####(5)]: url
  > \#####

* > [H######(6)]: url
  > \######

* > [H#######(7)]: url
  > #######

* > [H:]: url
  > :

* > [H```]: url
  > \`\`\`

* > [H~~~]: url
  > \~\~\~

* > [H*]: url
  > \*

* > [H+]: url
  > \+

* > [H-]: url
  > \-

* > [H***]: url
  > \*\*\*

* > [H___]: url
  > \_\_\_

* > [H---]: url
  > \-\-\-

* >> [I]: url 'title'
  >> \>  <

* >> [II]: url 'title'
  >> \>  <

* >> [III]: url 'title'
  >> \>  <

* >> [I#(1)]: url 'title'
  >> \#

* >> [I##(2)]: url 'title'
  >> \##

* >> [I###(3)]: url 'title'
  >> \###

* >> [I####(4)]: url 'title'
  >> \####

* >> [I#####(5)]: url 'title'
  >> \#####

* >> [I######(6)]: url 'title'
  >> \######

* >> [I#######(7)]: url 'title'
  >> #######

* >> [I:]: url 'title'
  >> :

* >> [I```]: url 'title'
  >> \`\`\`

* >> [I~~~]: url 'title'
  >> \~\~\~

* >> [I*]: url 'title'
  >> \*

* >> [I+]: url 'title'
  >> \+

* >> [I-]: url 'title'
  >> \-

* >> [I***]: url 'title'
  >> \*\*\*

* >> [I___]: url 'title'
  >> \_\_\_

* >> [I---]: url 'title'
  >> \-\-\-

* >> [J]: url
  >> \>  <

* >> [JJ]: url
  >> \>  <

* >> [J#(1)]: url
  >> \#

* >> [J##(2)]: url
  >> \##

* >> [J###(3)]: url
  >> \###

* >> [J####(4)]: url
  >> \####

* >> [J#####(5)]: url
  >> \#####

* >> [J######(6)]: url
  >> \######

* >> [J#######(7)]: url
  >> #######

* >> [J:]: url
  >> :

* >> [J```]: url
  >> \`\`\`

* >> [J~~~]: url
  >> \~\~\~

* >> [J*]: url
  >> \*

* >> [J+]: url
  >> \+

* >> [J-]: url
  >> \-

* >> [J***]: url
  >> \*\*\*

* >> [J___]: url
  >> \_\_\_

* >> [J---]: url
  >> \-\-\-


> [K-lazy]: url 'title'
> \>  <

> [KK-lazy]: url 'title'
> \>  <

> [K3-lazy]: url 'title'
> \>  <

> [K#(1)-lazy]: url 'title'
> \#

> [K##(2)-lazy]: url 'title'
> \##

> [K###(3)-lazy]: url 'title'
> \###

> [K####(4)]-lazy: url 'title'
> \####

> [K#####(5)-lazy]: url 'title'
> \#####

> [K######(6)-lazy]: url 'title'
> \######

> [K#######(7)-lazy]: url 'title'
> #######

> [K:-lazy]: url 'title'
> :

> [K```-lazy]: url 'title'
> \`\`\`

> [K~~~-lazy]: url 'title'
> \~\~\~

> [K*-lazy]: url 'title'
> \*

> [K+-lazy]: url 'title'
> \+

> [K--lazy]: url 'title'
> \-

> [K***-lazy]: url 'title'
> \*\*\*

> [K___-lazy]: url 'title'
> \_\_\_

> [K----lazy]: url 'title'
> \-\-\-

> [L-lazy]: url
> \>  <

> [LL-lazy]: url
> \>  <

> [L#(1)-lazy]: url
> \#

> [L##(2)-lazy]: url
> \##

> [L###(3)-lazy]: url
> \###

> [L####(4)-lazy]: url
> \####

> [L#####(5)-lazy]: url
> \#####

> [L######(6)-lazy]: url
> \######

> [L#######(7)-lazy]: url
> #######

> [L:-lazy]: url
> :

> [L```-lazy]: url
> \`\`\`

> [L~~~-lazy]: url
> \~\~\~

> [L*-lazy]: url
> \*

> [L+-lazy]: url
> \+

> [L--lazy]: url
> \-

> [L***-lazy]: url
> \*\*\*

> [L___-lazy]: url
> \_\_\_

> [L----lazy]: url
> \-\-\-

* > [M-lazy]: url 'title'
  > \>  <

* > [MM-lazy]: url 'title'
  > \>  <

* > [MMM-lazy]: url 'title'
  > \>  <

* > [M#(1)-lazy]: url 'title'
  > \#

* > [M##(2)-lazy]: url 'title'
  > \##

* > [M###(3)-lazy]: url 'title'
  > \###

* > [M####(4)-lazy]: url 'title'
  > \####

* > [M#####(5)-lazy]: url 'title'
  > \#####

* > [M######(6)-lazy]: url 'title'
  > \######

* > [M#######(7)-lazy]: url 'title'
  > #######

* > [M:-lazy]: url 'title'
  > :

* > [M```-lazy]: url 'title'
  > \`\`\`

* > [M~~~-lazy]: url 'title'
  > \~\~\~

* > [M*-lazy]: url 'title'
  > \*

* > [M+-lazy]: url 'title'
  > \+

* > [M--lazy]: url 'title'
  > \-

* > [M***-lazy]: url 'title'
  > \*\*\*

* > [M___-lazy]: url 'title'
  > \_\_\_

* > [M----lazy]: url 'title'
  > \-\-\-

* > [N-lazy]: url
  > \>  <

* > [NN-lazy]: url
  > \>  <

* > [N#(1)-lazy]: url
  > \#

* > [N##(2)-lazy]: url
  > \##

* > [N###(3)-lazy]: url
  > \###

* > [N####(4)-lazy]: url
  > \####

* > [N#####(5)-lazy]: url
  > \#####

* > [N######(6)-lazy]: url
  > \######

* > [N#######(7)-lazy]: url
  > #######

* > [N:-lazy]: url
  > :

* > [N```-lazy]: url
  > \`\`\`

* > [N~~~-lazy]: url
  > \~\~\~

* > [N*-lazy]: url
  > \*

* > [N+-lazy]: url
  > \+

* > [N--lazy]: url
  > \-

* > [N***-lazy]: url
  > \*\*\*

* > [N___-lazy]: url
  > \_\_\_

* > [N----lazy]: url
  > \-\-\-

* >> [O-lazy]: url 'title'
  >> \>  <

* >> [OO-lazy]: url 'title'
  >> \>  <

* >> [OOO-lazy]: url 'title'
  >> \>  <

* >> [O#(1)-lazy]: url 'title'
  >> \#

* >> [O##(2)-lazy]: url 'title'
  >> \##

* >> [O###(3)-lazy]: url 'title'
  >> \###

* >> [O####(4)-lazy]: url 'title'
  >> \####

* >> [O#####(5)-lazy]: url 'title'
  >> \#####

* >> [O######(6)-lazy]: url 'title'
  >> \######

* >> [O#######(7)-lazy]: url 'title'
  >> #######

* >> [O:-lazy]: url 'title'
  >> :

* >> [O```-lazy]: url 'title'
  >> \`\`\`

* >> [O~~~-lazy]: url 'title'
  >> \~\~\~

* >> [O*-lazy]: url 'title'
  >> \*

* >> [O+-lazy]: url 'title'
  >> \+

* >> [O--lazy]: url 'title'
  >> \-

* >> [O***-lazy]: url 'title'
  >> \*\*\*

* >> [O___-lazy]: url 'title'
  >> \_\_\_

* >> [O----lazy]: url 'title'
  >> \-\-\-

* >> [P-lazy]: url
  >> \>  <

* >> [PP-lazy]: url
  >> \>  <

* >> [P#(1)-lazy]: url
  >> \#

* >> [P##(2)-lazy]: url
  >> \##

* >> [P###(3)-lazy]: url
  >> \###

* >> [P####(4)-lazy]: url
  >> \####

* >> [P#####(5)-lazy]: url
  >> \#####

* >> [P######(6)-lazy]: url
  >> \######

* >> [P#######(7)-lazy]: url
  >> #######

* >> [P:-lazy]: url
  >> :

* >> [P```-lazy]: url
  >> \`\`\`

* >> [P~~~-lazy]: url
  >> \~\~\~

* >> [P*-lazy]: url
  >> \*

* >> [P+-lazy]: url
  >> \+

* >> [P--lazy]: url
  >> \-

* >> [P***-lazy]: url
  >> \*\*\*

* >> [P___-lazy]: url
  >> \_\_\_

* >> [P----lazy]: url
  >> \-\-\-


* >> [Q-extra-lazy]: url 'title'
  >> \>  <

* >> [QQ-extra-lazy]: url 'title'
  >> \>  <

* >> [QQQ-extra-lazy]: url 'title'
  >> \>  <

* >> [Q#(1)-extra-lazy]: url 'title'
  >> \#

* >> [Q##(2)-extra-lazy]: url 'title'
  >> \##

* >> [Q###(3)-extra-lazy]: url 'title'
  >> \###

* >> [Q####(4)-extra-lazy]: url 'title'
  >> \####

* >> [Q#####(5)-extra-lazy]: url 'title'
  >> \#####

* >> [Q######(6)-extra-lazy]: url 'title'
  >> \######

* >> [Q#######(7)-extra-lazy]: url 'title'
  >> #######

* >> [Q:-extra-lazy]: url 'title'
  >> :

* >> [Q```-extra-lazy]: url 'title'
  >> \`\`\`

* >> [Q~~~-extra-lazy]: url 'title'
  >> \~\~\~

* >> [Q*-extra-lazy]: url 'title'
  >> \*

* >> [Q+-extra-lazy]: url 'title'
  >> \+

* >> [Q--extra-lazy]: url 'title'
  >> \-

* >> [Q***-extra-lazy]: url 'title'
  >> \*\*\*

* >> [Q___-extra-lazy]: url 'title'
  >> \_\_\_

* >> [Q----extra-lazy]: url 'title'
  >> \-\-\-

* >> [R-extra-lazy]: url
  >> \>  <

* >> [RR-extra-lazy]: url
  >> \>  <

* >> [R#(1)-extra-lazy]: url
  >> \#

* >> [R##(2)-extra-lazy]: url
  >> \##

* >> [R###(3)-extra-lazy]: url
  >> \###

* >> [R####(4)-extra-lazy]: url
  >> \####

* >> [R#####(5)-extra-lazy]: url
  >> \#####

* >> [R######(6)-extra-lazy]: url
  >> \######

* >> [R#######(7)-extra-lazy]: url
  >> #######

* >> [R:-extra-lazy]: url
  >> :

* >> [R```-extra-lazy]: url
  >> \`\`\`

* >> [R~~~-extra-lazy]: url
  >> \~\~\~

* >> [R*-extra-lazy]: url
  >> \*

* >> [R+-extra-lazy]: url
  >> \+

* >> [R--extra-lazy]: url
  >> \-

* >> [R***-extra-lazy]: url
  >> \*\*\*

* >> [R___-extra-lazy]: url
  >> \_\_\_

* >> [R----extra-lazy]: url
  >> \-\-\-

> * [S]: url 'title'
>   \>  <

> * [SS]: url 'title'
>   \>  <

> * [SSS]: url 'title'
>   \>  <

> * [S#(1)]: url 'title'
>   \#

> * [S##(2)]: url 'title'
>   \##

> * [S###(3)]: url 'title'
>   \###

> * [S####(4)]: url 'title'
>   \####

> * [S#####(5)]: url 'title'
>   \#####

> * [S######(6)]: url 'title'
>   \######

> * [S#######(7)]: url 'title'
>   #######

> * [S:]: url 'title'
>   :

> * [S```]: url 'title'
>   \`\`\`

> * [S~~~]: url 'title'
>   \~\~\~

> * [S*]: url 'title'
>   \*

> * [S+]: url 'title'
>   \+

> * [S-]: url 'title'
>   \-

> * [S***]: url 'title'
>   \*\*\*

> * [S___]: url 'title'
>   \_\_\_

> * [S---]: url 'title'
>   \-\-\-

> * [T]: url
>   \>  <

> * [T]: url
>   \>  <

> * [T#(1)]: url
>   \#

> * [T##(2)]: url
>   \##

> * [T###(3)]: url
>   \###

> * [T####(4)]: url
>   \####

> * [T#####(5)]: url
>   \#####

> * [T######(6)]: url
>   \######

> * [T#######(7)]: url
>   #######

> * [T:]: url
>   :

> * [T```]: url
>   \`\`\`

> * [T~~~]: url
>   \~\~\~

> * [T*]: url
>   \*

> * [T+]: url
>   \+

> * [T-]: url
>   \-

> * [T***]: url
>   \*\*\*

> * [T___]: url
>   \_\_\_

> * [T---]: url
>   \-\-\-

> * > [U]: url 'title'
>   > \>  <

> * > [UU]: url 'title'
>   > \>  <

> * > [U]: url 'title'
>   > \>  <

> * > [U#(1)]: url 'title'
>   > \#

> * > [U##(2)]: url 'title'
>   > \##

> * > [U###(3)]: url 'title'
>   > \###

> * > [U####(4)]: url 'title'
>   > \####

> * > [U#####(5)]: url 'title'
>   > \#####

> * > [U######(6)]: url 'title'
>   > \######

> * > [U#######(7)]: url 'title'
>   > #######

> * > [U:]: url 'title'
>   > :

> * > [U```]: url 'title'
>   > \`\`\`

> * > [U~~~]: url 'title'
>   > \~\~\~

> * > [U*]: url 'title'
>   > \*

> * > [U+]: url 'title'
>   > \+

> * > [U-]: url 'title'
>   > \-

> * > [U***]: url 'title'
>   > \*\*\*

> * > [U___]: url 'title'
>   > \_\_\_

> * > [U---]: url 'title'
>   > \-\-\-

> * > [V]: url
>   > \>  <

> * > [VV]: url
>   > \>  <

> * > [V#(1)]: url
>   > \#

> * > [V##(2)]: url
>   > \##

> * > [V###(3)]: url
>   > \###

> * > [V####(4)]: url
>   > \####

> * > [V#####(5)]: url
>   > \#####

> * > [V######(6)]: url
>   > \######

> * > [V#######(7)]: url
>   > #######

> * > [V:]: url
>   > :

> * > [V```]: url
>   > \`\`\`

> * > [V~~~]: url
>   > \~\~\~

> * > [V*]: url
>   > \*

> * > [V+]: url
>   > \+

> * > [V-]: url
>   > \-

> * > [V***]: url
>   > \*\*\*

> * > [V___]: url
>   > \_\_\_

> * > [V---]: url
>   > \-\-\-

> * [W-lazy]: url 'title'
>   \>  <

> * [WW-lazy]: url 'title'
>   \>  <

> * [WWW-lazy]: url 'title'
>   \>  <

> * [W#(1)-lazy]: url 'title'
>   \#

> * [W##(2)-lazy]: url 'title'
>   \##

> * [W###(3)-lazy]: url 'title'
>   \###

> * [W####(4)-lazy]: url 'title'
>   \####

> * [W#####(5)-lazy]: url 'title'
>   \#####

> * [W######(6)-lazy]: url 'title'
>   \######

> * [W#######(7)-lazy]: url 'title'
>   #######

> * [W:-lazy]: url 'title'
>   :

> * [W```-lazy]: url 'title'
>   \`\`\`

> * [W~~~-lazy]: url 'title'
>   \~\~\~

> * [W*-lazy]: url 'title'
>   \*

> * [W+-lazy]: url 'title'
>   \+

> * [W--lazy]: url 'title'
>   \-

> * [W***-lazy]: url 'title'
>   \*\*\*

> * [W___-lazy]: url 'title'
>   \_\_\_

> * [W----lazy]: url 'title'
>   \-\-\-

> * [X-lazy]: url
>   \>  <

> * [XX-lazy]: url
>   \>  <

> * [X#(1)-lazy]: url
>   \#

> * [X##(2)-lazy]: url
>   \##

> * [X###(3)-lazy]: url
>   \###

> * [X####(4)-lazy]: url
>   \####

> * [X#####(5)-lazy]: url
>   \#####

> * [X######(6)-lazy]: url
>   \######

> * [X#######(7)-lazy]: url
>   #######

> * [X:-lazy]: url
>   :

> * [X```-lazy]: url
>   \`\`\`

> * [X~~~-lazy]: url
>   \~\~\~

> * [X*-lazy]: url
>   \*

> * [X+-lazy]: url
>   \+

> * [X--lazy]: url
>   \-

> * [X***-lazy]: url
>   \*\*\*

> * [X___-lazy]: url
>   \_\_\_

> * [X----lazy]: url
>   \-\-\-

> * > [Y-lazy]: url 'title'
>   > \>  <

> * > [YY-lazy]: url 'title'
>   > \>  <

> * > [Y-lazy]: url 'title'
>   > \>  <

> * > [Y#(1)-lazy]: url 'title'
>   > \#

> * > [Y##(2)-lazy]: url 'title'
>   > \##

> * > [Y###(3)-lazy]: url 'title'
>   > \###

> * > [Y####(4)-lazy]: url 'title'
>   > \####

> * > [Y#####(5)-lazy]: url 'title'
>   > \#####

> * > [Y######(6)-lazy]: url 'title'
>   > \######

> * > [Y#######(7)-lazy]: url 'title'
>   > #######

> * > [Y:-lazy]: url 'title'
>   > :

> * > [Y```-lazy]: url 'title'
>   > \`\`\`

> * > [Y~~~-lazy]: url 'title'
>   > \~\~\~

> * > [Y*-lazy]: url 'title'
>   > \*

> * > [Y+-lazy]: url 'title'
>   > \+

> * > [Y--lazy]: url 'title'
>   > \-

> * > [Y***-lazy]: url 'title'
>   > \*\*\*

> * > [Y___-lazy]: url 'title'
>   > \_\_\_

> * > [Y----lazy]: url 'title'
>   > \-\-\-

> * > [Z-lazy]: url
>   > \>  <

> * > [ZZ-lazy]: url
>   > \>  <

> * > [Z#(1)-lazy]: url
>   > \#

> * > [Z##(2)-lazy]: url
>   > \##

> * > [Z###(3)-lazy]: url
>   > \###

> * > [Z####(4)-lazy]: url
>   > \####

> * > [Z#####(5)-lazy]: url
>   > \#####

> * > [Z######(6)-lazy]: url
>   > \######

> * > [Z#######(7)-lazy]: url
>   > #######

> * > [Z:-lazy]: url
>   > :

> * > [Z```-lazy]: url
>   > \`\`\`

> * > [Z~~~-lazy]: url
>   > \~\~\~

> * > [Z*-lazy]: url
>   > \*

> * > [Z+-lazy]: url
>   > \+

> * > [Z--lazy]: url
>   > \-

> * > [Z***-lazy]: url
>   > \*\*\*

> * > [Z___-lazy]: url
>   > \_\_\_

> * > [Z----lazy]: url
>   > \-\-\-

> * > [a-extra-lazy]: url 'title'
>   > \>  <

> * > [aa-extra-lazy]: url 'title'
>   > \>  <

> * > [aaa-extra-lazy]: url 'title'
>   > \>  <

> * > [a#(1)-extra-lazy]: url 'title'
>   > \#

> * > [a##(2)-extra-lazy]: url 'title'
>   > \##

> * > [a###(3)-extra-lazy]: url 'title'
>   > \###

> * > [a####(4)-extra-lazy]: url 'title'
>   > \####

> * > [a#####(5)-extra-lazy]: url 'title'
>   > \#####

> * > [a######(6)-extra-lazy]: url 'title'
>   > \######

> * > [a#######(7)-extra-lazy]: url 'title'
>   > #######

> * > [a:-extra-lazy]: url 'title'
>   > :

> * > [a```-extra-lazy]: url 'title'
>   > \`\`\`

> * > [a~~~-extra-lazy]: url 'title'
>   > \~\~\~

> * > [a*-extra-lazy]: url 'title'
>   > \*

> * > [a+-extra-lazy]: url 'title'
>   > \+

> * > [a--extra-lazy]: url 'title'
>   > \-

> * > [a***-extra-lazy]: url 'title'
>   > \*\*\*

> * > [a___-extra-lazy]: url 'title'
>   > \_\_\_

> * > [a----extra-lazy]: url 'title'
>   > \-\-\-

> * > [b-extra-lazy]: url
>   > \>  <

> * > [bb-extra-lazy]: url
>   > \>  <

> * > [b#(1)-extra-lazy]: url
>   > \#

> * > [b##(2)-extra-lazy]: url
>   > \##

> * > [b###(3)-extra-lazy]: url
>   > \###

> * > [b####(4)-extra-lazy]: url
>   > \####

> * > [b#####(5)-extra-lazy]: url
>   > \#####

> * > [b######(6)-extra-lazy]: url
>   > \######

> * > [b#######(7)-extra-lazy]: url
>   > #######

> * > [b:-extra-lazy]: url
>   > :

> * > [b```-extra-lazy]: url
>   > \`\`\`

> * > [b~~~-extra-lazy]: url
>   > \~\~\~

> * > [b*-extra-lazy]: url
>   > \*

> * > [b+-extra-lazy]: url
>   > \+

> * > [b--extra-lazy]: url
>   > \-

> * > [b***-extra-lazy]: url
>   > \*\*\*

> * > [b___-extra-lazy]: url
>   > \_\_\_

> * > [b----extra-lazy]: url
>   > \-\-\-

<!-- escape what looks like a footnote reference -->
Some text
[\^  p] -- not a footnote reference

Some text
[\^ [] p] -- not a footnote reference

<!-- Escape `^` in link reference definition  -->

[\^.]: /url


<!--
Add a dummy escape header when we parse a definition list without a title.
This keeps the output idempotent.
See https://github.com/pulldown-cmark/pulldown-cmark/issues/997
-->

[a]: /url
<!-- Did you mean for this to be a definiton list? -->
<!-- If not, you should escape the `:` below -->
\\

:
     1
     2
     3

> [b]: /url
> <!-- Did you mean for this to be a definiton list? -->
> <!-- If not, you should escape the `:` below -->
> \\
> :
>      1
>      2
>      3

[c]: [.]:

<!-- Did you mean for this to be a definiton list? -->
<!-- If not, you should escape the `:` below -->
\\

:

<!-- Don't absorb code block into definition list -->
<!-- Consider a fenced code block instead -->
    ]

> [d]: [.]:
> <!-- Did you mean for this to be a definiton list? -->
> <!-- If not, you should escape the `:` below -->
> \\
> :
>
> <!-- Don't absorb code block into definition list -->
> <!-- Consider a fenced code block instead -->
>     ]

* > [e]: [.]:
  > <!-- Did you mean for this to be a definiton list? -->
  > <!-- If not, you should escape the `:` below -->
  > \\
  > :
  >
  > <!-- Don't absorb code block into definition list -->
  > <!-- Consider a fenced code block instead -->
  >     ]

[f]: [.]:

<!-- Did you mean for this to be a definiton list? -->
<!-- If not, you should escape the `:` below -->
\\

:

    ]

> [g]: [.]:
> <!-- Did you mean for this to be a definiton list? -->
> <!-- If not, you should escape the `:` below -->
> \\
> :
>
>     ]

* > [h]: [.]:
  > <!-- Did you mean for this to be a definiton list? -->
  > <!-- If not, you should escape the `:` below -->
  > \\
  > :
  >
  >     ]

<!-- Escape text that looks like an ordered list -->
a
\1) text

b
\01) text

c
\001) text

d
\1. text

e
\01. text

f
\001. text

<!-- Don't need to escape number besides 1 -->

a
2) text

b
02) text

c
002) text

d
2. text

e
02. text

f
002. text

<!-- Don't need to escape text if it's an empty list -->

a
1)

b
01)

c
001)

d
1.

e
01.

f
001.

<!-- Escape table delimiter row in Header. This is a little more aggressive than it needs to be -->
H1(A)
\-|
=

H2(B)
\-|
-

<!-- Escape table delimiter row in Definition List Title. This is a little more aggressive than it needs to be -->

C
\-|
:

<!-- Escape table delimiter row in Paragraph because previous line has a `|` character.  -->

D |
\-|

<!-- Don't need to escape the delimiter row in the Paragraph because the previous row doesn't have a `|` character -->

E
-|


<!-- Need to escape the `:` if it might later get interpreted as a link reference definition -->
> [A]\:        
> \=

> [B]\:        
> \=

> [C]\:        
> \=

> leadin text [D]\:        
> \=

> [E]\:        
> = with extra text

<!-- Escape trailing escape in footnote definition -->
[^\`@%\@\\]:
    [       `~z)C\`
