[zero]: /zero-url "zero-title"

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

1.
   [seven-point-one]: /seven-point-one-url "seven-point-one-title"

[seve-point-one]

[eight]
- [eight]: /eight-url "eight-title"

[eight-point-one]

1.
   [eight-point-one]: /eight-point-one-url "eight-point-one-title"

# duplicate reference definitions
[nine]
[nine]: /nine-first-url "nine-first-title"
[nine]: /nine-second-url "nine-second-title"

# reference definition without a link
[ten]: /ten-url "ten-url"

# Deeply nested reference definitions
>
> [eleven]: /eleven-url
>
>>
>> [twelve]: </twelve-url> (twelve-title)
>>
>>
>>> [thirteen]: </thirteen-url> 'thirteen-title'
>>>
>>>> [eleven]
>>>> [twelve]
>>>> [thirteen]

> * [fourteen]
>   >
>   > [fourteen]: fourteen-url 'fourteen-title'
>   >
>   > *
>   > *
>   >   *
>   >   * [fifteen]: /fifteen-url (fifteen-title)
>   >     + [fifteen]

# I tried defining the reference in a table. I don't think it works
| col 1     | col 2 |
| --------- | ----- |
| [sixteen] |       |
|           |       |

[sixteen]: /sixteen-url 'sixteen-title'

# emojis!
[7️⃣-teen]

[7️⃣-teen]: 7️⃣-teen-url '7️⃣-teen-title'

<!-- Odd Cases found when fuzzing -->
[.]: []:[]

<!-- recover link reference defintions before a rule -->

[.]: a
***

[.]: b

***
