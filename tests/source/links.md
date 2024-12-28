[one]

[ two]

[ three ]

[ four ][]

[ `five` ][ `five` ]

[ `six` ]( /url )

[** seven **](
    /url)

[
    ~eight~
]

[
    some
    multi
    lined
    text
]

[  link   *foo **bar** `#`*](/uri)

![nine]

![ ten]

![ eleven ]

![ `twelve` ]

![ `thirteen` ][ `thirteen` ]

![ `fourteen` ]( /url )

![** fifteen **](
    /url)

![
    ~sixteen~
]

![
    some
    multi
    lined
    text
]


[ reference definition ]: /some/url
[
    another reference definition
]:   /some/url

<!-- multi-line reference link label -->
- [][p
^]X

<!-- Multi-line code in link is removed -->
[`foo
` bar]

<!-- Hard break in link is removed -->
[some text  
<]

[some text2\
<]

<!-- Properly parse label in reference links when there's an escape -->
 [*][^\ ][q]
