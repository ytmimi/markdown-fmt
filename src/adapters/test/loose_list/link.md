- [some link](/url)

==========

event=Start(List(None)) range=0..21
event=Start(Item) range=0..21
event=Start(Paragraph) range=2..19
event=Start(Link(Inline, Borrowed("/url"), Borrowed(""))) range=2..19
event=Text(Borrowed("some link")) range=3..12
event=End(Link(Inline, Borrowed("/url"), Borrowed(""))) range=2..19
event=End(Paragraph) range=2..19
event=End(Item) range=0..21
event=End(List(None)) range=0..21
