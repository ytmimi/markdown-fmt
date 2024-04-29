# reference definition at the start of a block quote
> [one]: /one-url "one-title"
>
> [one]
>
>

# reference definition at the end of a block quote
>
> [two]
>
> [two]: /two-url "two-title"
>

# reference definition at the start of a list item
* [three]: /three-url "three-title"
  [three]


# reference definition at the end of a list item
* [four]

  [four]: /four-url "four-title"


# reference definition in block quote, but link outside
> [five]: /five-url "five-title"
[five]

[six]
> [six]: /six-url "six-title"


# reference definition in list item, but link outside
- [seven]: /seven-url "seven-title"
[seven]

[eight]
- [eight]: /eight-url "eight-title"

# duplicate reference definitions
[nine]
[nine]: /nine-first-url "nine-first-title"
[nine]: /nine-second-url "nine-second-title"

# reference definition without a link
[ten]: /ten-url "ten-url"
