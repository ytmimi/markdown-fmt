> `start of code
> end of clode`

> ``start of code
> end of clode``

<!-- Don't escape ``` for code even though it looks like the opening of a code fence -->
$```
1```


<!-- This get's parsed as multi-line code even though it starts with (```)-->

``` ` 
 `$```~^

>>`
|`

>  > `
>  > |`

<!--
    Instead of escaping multi-line code with lazy-continuations indent what would have been
    escpaed by 4 spaces.
-->
* - `start of code
--
end of code`
