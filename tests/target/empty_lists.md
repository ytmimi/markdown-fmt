*
  *
    *

-
  -
    -

+
  +
    +

*
  *
  +
  -

+
  +
    +
    -
    -
  +
  -
  -
  -
  +
    -
    -
  *
*

1.
1.
1.

1)
1)
1)

1.
2.
3.

1)
2)
3)

1.
   2.
      3.
         4.
            5.
               6.
                  7)
                     8)
                        9)
                           10)
                               11) *
                                     -
                                     -
                                     +
                           12)
                        13)
                     14)
                  15)
               16)
            17)
         18)
      19)
   20)
21)

00)
01)
02)
03)
04)
05)
06)
07)
08)
09)
010)

00.
01.
02.
03.
04.
05.
06.
07.
08.
09.
010.

<!-- case with tabs (found when fuzzing)
     To prevent the `-` from getting interpreted as a setext header the list is given another
     newline separator.
-->

*[
-       +*[
  [

  -
    -z*

<!-- Tight list that starts with a hard break should be idempotent -->
* \
  ~

<!-- list with emphasis -->
*
  *A*
  A


+
  _B_
  B


-
  *C*
  C


*
  _D_
  D


+
  *E*
  E


-
  _F_
  F

<!-- list with strong emphasis -->

*
  **G**
  G


+
  __H__
  H


-
  **I**
  I


*
  __J__
  J


+
  **K**
  K


-
  __L__
  L

<!-- list with strikethrough -->
*
  ~M~
  M


+
  ~~N~~
  N


-
  ~~O~~
  O


*
  ~~P~~
  P


+
  ~~Q~~
  Q


-
  ~~R~~
  R
