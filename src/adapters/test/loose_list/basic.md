* a
  b

==========

event=Start(List(None)) range=0..9
event=Start(Item) range=0..9
event=Start(Paragraph) range=2..7
event=Text(Borrowed("a")) range=2..3
event=SoftBreak range=3..4
event=Text(Borrowed("b")) range=6..7
event=End(Paragraph) range=2..7
event=End(Item) range=0..9
event=End(List(false)) range=0..9
