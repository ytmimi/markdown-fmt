pub(crate) type CodeBlockFormatter = Box<dyn Fn(&str, String) -> String>;

/// Builder for the [MarkdownFormatter](crate::MarkdownFormatter)
pub struct FormatterBuilder {
    code_block_formatter: CodeBlockFormatter,
}

impl FormatterBuilder {
    /// Create a [FormatterBuilder] with a custom code block formatter.
    ///
    /// The closure used to reformat code blocks takes two arguments;
    /// the [`info string`] and the complete code snippet
    ///
    /// ```rust
    /// # use markdown_fmt::MarkdownFormatter;
    /// # use markdown_fmt::FormatterBuilder;
    /// let builder = FormatterBuilder::with_code_block_formatter(|info_string, code_block| {
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
    /// # use markdown_fmt::FormatterBuilder;
    /// let builder = FormatterBuilder::default();
    /// let formatter: MarkdownFormatter = builder.build();
    /// ```
    pub fn build(self) -> crate::MarkdownFormatter {
        crate::MarkdownFormatter::new(self.code_block_formatter)
    }

    /// Configure how code blocks should be reformatted after creating the [FormatterBuilder].
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
}

impl std::fmt::Debug for FormatterBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FormatterBuilder")
    }
}

/// Default formatter which leaves code blocks unformatted
fn default_code_block_formatter(_info_str: &str, code_block: String) -> String {
    code_block
}

impl Default for FormatterBuilder {
    fn default() -> Self {
        FormatterBuilder {
            code_block_formatter: Box::new(default_code_block_formatter),
        }
    }
}
