use crate::config::Config;

/// Provides info that custom code block formatters can use
/// when formatting code.
#[derive(Debug, Clone)]
pub struct CodeBlockContext {
    pub(crate) indentation: usize,
    pub(crate) max_width: Option<usize>,
}

impl CodeBlockContext {
    /// Get the indented of the current code block relative to the left margin of the markdown
    /// snippet that's being formatted.
    ///
    /// ```````markdown
    /// <!-- indentation is 2 because of the list -->
    /// * ``````rust
    ///   fn m() {}
    ///   ``````
    /// <!-- indentation is 4 because of the list and block quote -->
    /// * > ``````rust
    ///   > fn m() {}
    ///   > ``````
    ///
    /// * > ``````markdown
    ///   > <!-- indentation is 0 because relative to the enclosing `markdown` block -->
    ///   > <!-- there is no indentation -->
    ///   > `````rust
    ///   > fn m() {}
    ///   > `````
    ///   > ``````
    /// ```````
    pub fn indentation(&self) -> usize {
        self.indentation
    }

    /// Get the `max_width` that's configured for this code block.
    ///
    /// returns [None] if [`max_width`] was not configured.
    ///
    /// [`max_width`]: FormatBuilder::max_width
    pub fn max_width(&self) -> Option<usize> {
        self.max_width
    }
}

pub(crate) type CodeBlockFormatter = Box<dyn Fn(&CodeBlockContext, &str, String) -> String>;

/// Builder for the [MarkdownFormatter](crate::MarkdownFormatter)
pub struct FormatBuilder {
    code_block_formatter: CodeBlockFormatter,
    config: Config,
}

impl FormatBuilder {
    /// Create a [FormatBuilder] with a custom code block formatter.
    ///
    /// The closure used to reformat code blocks takes three arguments;
    /// The [`CodeBlockContext`], [`info string`], and the complete code snippet.
    ///
    /// ```rust
    /// # use markdown_format::MarkdownFormatter;
    /// # use markdown_format::FormatBuilder;
    /// let builder = FormatBuilder::with_code_block_formatter(|_ctx, info_string, code_block| {
    ///     // Set the code block formatting logic
    ///     match info_string.to_lowercase().as_str() {
    ///         "rust" => {
    ///             // format rust code
    ///             # code_block
    ///         }
    ///         _ => code_block,
    ///     }
    /// });
    /// let formatter = builder.build();
    /// ```
    /// [`info string`]: https://spec.commonmark.org/0.31.2/#fenced-code-blocks
    pub fn with_code_block_formatter<F>(formatter: F) -> Self
    where
        F: Fn(&CodeBlockContext, &str, String) -> String + 'static,
    {
        let mut builder = Self::default();
        builder.code_block_formatter(formatter);
        builder
    }

    /// Build a [MarkdownFormatter](crate::MarkdownFormatter).
    ///
    /// ```rust
    /// # use markdown_format::MarkdownFormatter;
    /// # use markdown_format::FormatBuilder;
    /// let builder = FormatBuilder::default();
    /// let formatter: MarkdownFormatter = builder.build();
    /// ```
    pub fn build(self) -> crate::MarkdownFormatter {
        crate::MarkdownFormatter::new(self.code_block_formatter, self.config)
    }

    /// Configure how code blocks should be reformatted after creating the [FormatBuilder].
    ///
    /// The closure used to reformat code blocks takes three arguments;
    /// The [`CodeBlockContext`], [`info string`], and the complete code snippet.
    ///
    /// [`info string`]: https://spec.commonmark.org/0.31.2/#fenced-code-blocks
    pub fn code_block_formatter<F>(&mut self, formatter: F) -> &mut Self
    where
        F: Fn(&CodeBlockContext, &str, String) -> String + 'static,
    {
        self.code_block_formatter = Box::new(formatter);
        self
    }

    /// Configure the max width when rewriting paragraphs.
    ///
    /// When set to [None], the deafault, paragraph width is left unchanged.
    ///
    /// # Setting [`max_width`](Self::max_width) to [None] (default)
    ///
    /// ```rust
    /// # use markdown_format::FormatBuilder;
    /// let mut builder = FormatBuilder::default();
    /// builder.max_width(None);
    ///
    /// let input = "this text should not wrap";
    /// let output = builder.build().format(input).unwrap();
    /// assert_eq!(output, input)
    /// ```
    /// ---
    /// # Setting [`max_width`](Self::max_width) to `20`
    /// ```rust
    /// # use markdown_format::FormatBuilder;
    /// let mut builder = FormatBuilder::default();
    /// builder.max_width(Some(20));
    ///
    /// let input = "this text should definetly wrap";
    ///
    /// let expected = "this text should
    /// definetly wrap";
    /// let output = builder.build().format(input).unwrap();
    /// assert_eq!(output, expected)
    /// ```
    pub fn max_width(&mut self, max_width: Option<usize>) -> &mut Self {
        self.config.set_max_width(max_width);
        self
    }

    /// Configure whether or not paragraph text should reflow when `max_width`
    /// is also configured. By default text will not reflow.
    ///
    /// # Setting [`reflow_text`](Self::reflow_text) to `false` (default)
    /// ```rust
    /// # use markdown_format::FormatBuilder;
    /// let mut builder = FormatBuilder::default();
    /// builder.max_width(Some(30)).reflow_text(false);
    ///
    /// let input = "this
    /// will not
    /// reflow";
    ///
    /// let output = builder.build().format(input).unwrap();
    /// assert_eq!(output, input)
    /// ```
    /// ---
    /// # Setting [`reflow_text`](Self::reflow_text) to `true`
    /// ```rust
    /// # use markdown_format::FormatBuilder;
    /// let mut builder = FormatBuilder::default();
    /// builder.max_width(Some(30)).reflow_text(true);
    ///
    /// let input = "this
    /// text
    /// should
    /// reflow";
    ///
    /// let expected = "this text should reflow";
    /// let output = builder.build().format(input).unwrap();
    /// assert_eq!(output, expected)
    /// ```
    pub fn reflow_text(&mut self, should_reflow: bool) -> &mut Self {
        self.config.set_reflow_text(should_reflow);
        self
    }

    /// Internal setter for Config. Used for testing
    #[cfg(test)]
    pub(crate) fn config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }
}

impl std::fmt::Debug for FormatBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FormatBuilder")
    }
}

impl Default for FormatBuilder {
    fn default() -> Self {
        FormatBuilder {
            code_block_formatter: Box::new(|_ctx, _info_str, code_block| code_block),
            config: Config::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CHECK_FORMATTING_CONTEXT_INPUT: &str = "# test

```level_0
```

* ```level_1
  ```

* ~~~~markdown
  * ```level_2
    ```
  + >>>>> ```level_3
    >>>>> ```
  ~~~~

~~~~markdown
```level_4
```
~~~~
";

    #[test]
    fn check_formatting_context_with_max_width() {
        let mut builder = FormatBuilder::with_code_block_formatter(|ctx, info_str, code_block| {
            match info_str {
                "level_0" => {
                    assert_eq!(ctx.indentation(), 0);
                    assert_eq!(ctx.max_width(), Some(50));
                }
                "level_1" => {
                    assert_eq!(ctx.indentation(), 2);
                    assert_eq!(ctx.max_width(), Some(50));
                }
                "level_2" => {
                    assert_eq!(ctx.indentation(), 2);
                    assert_eq!(ctx.max_width(), Some(48));
                }
                "level_3" => {
                    assert_eq!(ctx.indentation(), 8);
                    assert_eq!(ctx.max_width(), Some(48));
                }
                "level_4" => {
                    assert_eq!(ctx.indentation(), 0);
                    assert_eq!(ctx.max_width(), Some(50));
                }
                _ => panic!("unexpected info_str"),
            }
            code_block
        });
        builder.max_width(Some(50));

        let output = builder
            .build()
            .format(CHECK_FORMATTING_CONTEXT_INPUT)
            .unwrap();
        assert_eq!(CHECK_FORMATTING_CONTEXT_INPUT, output)
    }

    #[test]
    fn check_formatting_context_without_max_width() {
        let mut builder = FormatBuilder::with_code_block_formatter(|ctx, info_str, code_block| {
            match info_str {
                "level_0" => {
                    assert_eq!(ctx.indentation(), 0);
                    assert_eq!(ctx.max_width(), None);
                }
                "level_1" => {
                    assert_eq!(ctx.indentation(), 2);
                    assert_eq!(ctx.max_width(), None);
                }
                "level_2" => {
                    assert_eq!(ctx.indentation(), 2);
                    assert_eq!(ctx.max_width(), None);
                }
                "level_3" => {
                    assert_eq!(ctx.indentation(), 8);
                    assert_eq!(ctx.max_width(), None);
                }
                "level_4" => {
                    assert_eq!(ctx.indentation(), 0);
                    assert_eq!(ctx.max_width(), None);
                }
                _ => panic!("unexpected info_str"),
            }
            code_block
        });
        builder.max_width(None);

        let output = builder
            .build()
            .format(CHECK_FORMATTING_CONTEXT_INPUT)
            .unwrap();
        assert_eq!(CHECK_FORMATTING_CONTEXT_INPUT, output)
    }
}
