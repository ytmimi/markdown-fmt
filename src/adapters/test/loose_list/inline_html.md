-
  some html<br> some more stuff some
  <span><span>html</span> </span>
  <p> some more html</p>
  <p>
    some
    more
    html
  </p>

==========

event=Start(List(None)) range=0..139
event=Start(Item) range=0..139
event=Start(Paragraph) range=4..72
event=Text(Borrowed("some html")) range=4..13
event=InlineHtml(Borrowed("<br>")) range=13..17
event=Text(Borrowed(" some more stuff some")) range=17..38
event=SoftBreak range=38..39
event=InlineHtml(Borrowed("<span>")) range=41..47
event=InlineHtml(Borrowed("<span>")) range=47..53
event=Text(Borrowed("html")) range=53..57
event=InlineHtml(Borrowed("</span>")) range=57..64
event=Text(Borrowed(" ")) range=64..65
event=InlineHtml(Borrowed("</span>")) range=65..72
event=End(Paragraph) range=4..72
event=Start(HtmlBlock) range=75..138
event=Html(Borrowed("<p> some more html</p>\n")) range=75..98
event=Html(Borrowed("<p>\n")) range=100..104
event=Html(Borrowed("  some\n")) range=106..113
event=Html(Borrowed("  more\n")) range=115..122
event=Html(Borrowed("  html\n")) range=124..131
event=Html(Borrowed("</p>\n")) range=133..138
event=End(HtmlBlock) range=75..138
event=End(Item) range=0..139
event=End(List(false)) range=0..139
