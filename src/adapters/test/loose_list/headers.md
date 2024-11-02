- # Lorem ipsum dolor sit amet, consectetur adipiscing elit,
  some text after the heading

> - some text before the heading
>   # Lorem ipsum dolor sit amet, consectetur adipiscing elit,

==========

event=Start(List(None)) range=0..92
event=Start(Item) range=0..92
event=Start(Heading { level: H1, id: None, classes: [], attrs: [] }) range=2..61
event=Text(Borrowed("Lorem ipsum dolor sit amet, consectetur adipiscing elit,")) range=4..60
event=End(Heading(H1)) range=2..61
event=Start(Paragraph) range=63..90
event=Text(Borrowed("some text after the heading")) range=63..90
event=End(Paragraph) range=63..90
event=End(Item) range=0..92
event=End(List(false)) range=0..92
event=Start(BlockQuote(None)) range=92..188
event=Start(List(None)) range=94..188
event=Start(Item) range=94..188
event=Start(Paragraph) range=96..124
event=Text(Borrowed("some text before the heading")) range=96..124
event=End(Paragraph) range=96..124
event=Start(Heading { level: H1, id: None, classes: [], attrs: [] }) range=129..188
event=Text(Borrowed("Lorem ipsum dolor sit amet, consectetur adipiscing elit,")) range=131..187
event=End(Heading(H1)) range=129..188
event=End(Item) range=94..188
event=End(List(false)) range=94..188
event=End(BlockQuote) range=92..188
