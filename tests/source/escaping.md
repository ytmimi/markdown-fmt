<!-- Don't interpret as inline HTML -->

>*<!fJ<!fJ`
TT


<!-- Don't interpret as a table without a leading `|` -->

>6|
-|

<!-- space hard break followed by paragraph with single `-` -->
<  
    -

<!-- Don't interpret the '```' as the start of a fenced code block -->
--
    ```>

<!-- Don't interpret the '```' as the start of a fenced code block -->
--
    ~~~>

<!-- Don't interpret the `--` as a setext header -->
* -+
--
*-*>

<!-- Don't interpret the `==` as a setext header -->
* -+
==
*-*>
