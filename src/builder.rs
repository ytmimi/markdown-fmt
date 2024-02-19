pub(crate) type CodeBlockFormatter = Box<dyn Fn(&str, String) -> String>;

/// Builder for the [MarkdownFormatter](crate::MarkdownFormatter)
pub struct FormatterBuilder {
    code_block_formatter: CodeBlockFormatter,
}

impl FormatterBuilder {
    /// Create a [FormatterBuilder] with a custom code block formatter
    pub fn with_code_block_formatter<'a, F>(formatter: F) -> Self
    where
        F: Fn(&str, String) -> String + 'static,
    {
        let mut builder = Self::default();
        builder.code_block_formatter(formatter);
        builder
    }

    /// Try to build a [MarkdownFormatter](crate::MarkdownFormatter)
    pub fn build(self) -> Result<crate::MarkdownFormatter, FormatterBuilder> {
        Ok(crate::MarkdownFormatter::new(self.code_block_formatter))
    }

    pub fn code_block_formatter<'a, F>(&mut self, formatter: F) -> &mut Self
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
