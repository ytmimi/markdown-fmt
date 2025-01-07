<!-- 3. and 4. are not parsed as task list items -->

1. [x] done!
2. [ ] not done :(
3. [x]
4. [ ]


<!--
    Parser Bug. Manually escaping the text helps work around the issue.
    See https://github.com/pulldown-cmark/pulldown-cmark/issues/999
-->
- [x] * A

- [x] \* B

- [ ] 3. C

- [x] 4. - * 1) D
