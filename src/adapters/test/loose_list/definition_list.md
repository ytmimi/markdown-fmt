One
: some text


Two

: even
  more
   text

Three
:
 <span>some inline HTML</span>

==========

event=Start(DefinitionList) range=0..86
event=Start(DefinitionListTitle) range=0..4
event=Text(Borrowed("One")) range=0..3
event=End(DefinitionListTitle) range=0..4
event=Start(DefinitionListDefinition) range=4..18
event=Start(Paragraph) range=6..16
event=Text(Borrowed("some text")) range=6..15
event=End(Paragraph) range=6..16
event=End(DefinitionListDefinition) range=4..18
event=Start(DefinitionListTitle) range=18..22
event=Text(Borrowed("Two")) range=18..21
event=End(DefinitionListTitle) range=18..22
event=Start(DefinitionListDefinition) range=23..46
event=Start(Paragraph) range=25..45
event=Text(Borrowed("even")) range=25..29
event=SoftBreak range=29..30
event=Text(Borrowed("more")) range=32..36
event=SoftBreak range=36..37
event=Text(Borrowed("text")) range=40..44
event=End(Paragraph) range=25..45
event=End(DefinitionListDefinition) range=23..46
event=Start(DefinitionListTitle) range=46..52
event=Text(Borrowed("Three")) range=46..51
event=End(DefinitionListTitle) range=46..52
event=Start(DefinitionListDefinition) range=52..86
event=Start(Paragraph) range=55..85
event=InlineHtml(Borrowed("<span>")) range=55..61
event=Text(Borrowed("some inline HTML")) range=61..77
event=InlineHtml(Borrowed("</span>")) range=77..84
event=End(Paragraph) range=55..85
event=End(DefinitionListDefinition) range=52..86
event=End(DefinitionList) range=0..86
