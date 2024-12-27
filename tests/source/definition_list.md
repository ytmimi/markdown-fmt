<!-- empty definition list  -->
empty!
  :

<!-- empty with a reference link definition -->
empty, but I have a link definition!
  : [label]: /url "title"

<!-- empty with a reference link definition on the next line -->
link definition on the next line!
  :
    [label]: /url "title"

<!-- trailing link definition -->
trailing link definition
  : hey there!

    [label]: /url "title"


<!-- middle link definition -->
middle link definition
  :
    top!

    [label]: /url "title"

    bottom

<!-- indented code block in definition -->
check out the code block
  :
        print("first definition!")


Sibling definition list that also has a code block
  :
    top!

        print("second definition!")


<!-- definition list inside a definition list -->

level one!
  :
    level two!
      :
        level three!
          :


<!-- definition list inside a definition list with link definitions -->

level one!
  :
    [start label1]: /url "title"

    level two!
      :
        [start label2]: /url "title"

        level three!
          :
            [start label3]: /url "title"

level one!
:
    [start label1]: /url "title"

    level two!
    :
        [start label2]: /url "title"

        level three!
        :
            [start label3]: /url "title"

            [end label3]: /url "title"

        [end label2]: /url "title"

    [end label1]: /url "title"



<!-- Nested definition list with code block -->

level one!
:     print("1")
      print("1 next line")

    level two!
    :     print("2")

        level three!
        :
             print("3")


<!-- definition list in a block quote -->

> level one!
> :     print("1")
>       print("1 next line")


<!-- definition list in a block quote -->

> block quote one!
> :     print("1")
>       print("1 next line")
>
>   [label]: /url "title"


> block quote two!
>  :     print("1")
>        print("1 next line")
>
> [label]: /url "title"

> * list in block quote!
>    :     print("1")
>          print("1 next line")
>
>   [label]: /url "title"

> block quote sibling one!
> :     print("1")
>       print("1 next line")
>
>   [label]: /url "title"
>
> block quote sibling two!
> :     print("2")
>       print("1 next line")
>
>   [label]: /url "title"


<!-- definition list in list -->

* list one!
  :     print("1")
        print("1 next line")

  [label]: /url "title"

* > block quote in a list!
  > :     print("1")
  >       print("1 next line")
  >
  > [label]: /url "title"


* list sibling one!
  :     print("1")
        print("1 next line")

    [label]: /url "title"

  list sibling two!
  :     print("2")
        print("1 next line")

    [label]: /url "title"


<!-- crazy nested -->

*
  +
    * outer definition list
      :
        [first outer label]: /url "title"
        some text
        >> * inner definition list
        >>   :
        >>     [first inner label]: /url "title"
        >>
        >>     * why not add another list?
        >>     *
        >>     [last inner label]: /url "title"

        [last outer label]: /url "title"

<!-- empyt definition list followed by paragraph -->

some title
:
new paragraph

<!-- Looks like a definition list but it's not -->
start of a paragraph
		:next line of a paragraph

<!--
  The amount of indentation of a fenced code block seems to be dependent on the relative position
  of the content. If the definition list contains text, then the amount of indentation needed to
  define a fenced code block is relative to where that text starts.
-->
A
:B

     fenced-code-block 

with URL
:[label]: url "title"

     fenced-code-block 

C
:
 D

     fenced-code-block 
     notice one less space of indentation than the cases above


<!-- Needs an extra space to separate the definition list from the blockquote -->
>
><Y|
:-|
+*
\UUU

:<!T
