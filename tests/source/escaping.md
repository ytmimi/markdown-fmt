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

<!-- Don't interpret `* ` as a list, which prevents (```) from being interpreted as a fenced code block-->
&
    * ```

<!-- Don't interpret `- ` as a list, which prevents (```) from being interpreted as a fenced code block-->
&
    - ```

<!-- Don't interpret `+ ` as a list, which prevents (```) from being interpreted as a fenced code block-->
&
    + ```

<!-- Tight list that starts with the text `\\` and a soft break.
     Escape the `\` so it's not considered a hard break on future runs
-->
* \ 
~

<!-- This is already escaped. Don't add any more escapes -->
[
\\[]
