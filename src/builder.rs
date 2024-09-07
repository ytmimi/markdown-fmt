use crate::config::Config;

pub(crate) type CodeBlockFormatter = Box<dyn Fn(&str, String) -> String>;

/// Builder for the [MarkdownFormatter](crate::MarkdownFormatter)
pub struct FormatBuilder {
    code_block_formatter: CodeBlockFormatter,
    config: Config,
}

impl FormatBuilder {
    /// Create a [FormatBuilder] with a custom code block formatter.
    ///
    /// The closure used to reformat code blocks takes two arguments;
    /// the [`info string`] and the complete code snippet
    ///
    /// ```rust
    /// # use markdown_fmt::MarkdownFormatter;
    /// # use markdown_fmt::FormatBuilder;
    /// let builder = FormatBuilder::with_code_block_formatter(|info_string, code_block| {
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
        F: Fn(&str, String) -> String + 'static,
    {
        let mut builder = Self::default();
        builder.code_block_formatter(formatter);
        builder
    }

    /// Build a [MarkdownFormatter](crate::MarkdownFormatter)
    ///
    /// ```rust
    /// # use markdown_fmt::MarkdownFormatter;
    /// # use markdown_fmt::FormatBuilder;
    /// let builder = FormatBuilder::default();
    /// let formatter: MarkdownFormatter = builder.build();
    /// ```
    pub fn build(self) -> crate::MarkdownFormatter {
        crate::MarkdownFormatter::new(self.code_block_formatter, self.config)
    }

    /// Configure how code blocks should be reformatted after creating the [FormatBuilder].
    ///
    /// The closure passed to `code_block_formatter` takes two arguments;
    /// the [`info string`] and the complete code snippet
    ///
    /// [`info string`]: https://spec.commonmark.org/0.31.2/#fenced-code-blocks
    pub fn code_block_formatter<F>(&mut self, formatter: F) -> &mut Self
    where
        F: Fn(&str, String) -> String + 'static,
    {
        self.code_block_formatter = Box::new(formatter);
        self
    }

    /// Configure the max with when rewriting paragraphs.
    ///
    /// When set to [None], the deafault, paragraph width is left unchanged.
    pub fn max_width(&mut self, max_width: Option<usize>) -> &mut Self {
        self.config.set_max_width(max_width);
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

/// Default formatter which leaves code blocks unformatted
fn default_code_block_formatter(_info_str: &str, code_block: String) -> String {
    code_block
}

impl Default for FormatBuilder {
    fn default() -> Self {
        FormatBuilder {
            code_block_formatter: Box::new(default_code_block_formatter),
            config: Config::default(),
        }
    }
}
