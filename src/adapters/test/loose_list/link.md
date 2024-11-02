- [some link](/url)
- ![some image link](/url)

==========

event=Start(List(None)) range=0..48
event=Start(Item) range=0..20
event=Start(Paragraph) range=2..19
event=Start(Link { link_type: Inline, dest_url: Borrowed("/url"), title: Borrowed(""), id: Borrowed("") }) range=2..19
event=Text(Borrowed("some link")) range=3..12
event=End(Link) range=2..19
event=End(Paragraph) range=2..19
event=End(Item) range=0..20
event=Start(Item) range=20..48
event=Start(Paragraph) range=22..46
event=Start(Image { link_type: Inline, dest_url: Borrowed("/url"), title: Borrowed(""), id: Borrowed("") }) range=22..46
event=Text(Borrowed("some image link")) range=24..39
event=End(Image) range=22..46
event=End(Paragraph) range=22..46
event=End(Item) range=20..48
event=End(List(false)) range=0..48
