pub mod options;

#[derive(Debug, Default)]
pub(crate) struct Config {
    max_width: Option<usize>,
    reflow_text: bool,
    #[cfg(any(feature = "unstable-configs", feature = "unordered-list-marker"))]
    pub(crate) unordered_list_marker: Option<options::UnorderedListMarkerConfig>,
}

/// An error occured when trying to set a config in a test
#[cfg(test)]
#[derive(Debug)]
pub(crate) enum ConfigSetError<'a> {
    /// The feature needed to run this test hasn't been enabled.
    /// This should be treated as a warning, and the test should be skipped.
    #[allow(dead_code)]
    MissingFeature(&'static str),
    /// Some unknown configuration option was referenced in the test.
    /// This should be treated as a hard error.
    UnknownConfig(&'a str),
}

impl Config {
    pub(crate) fn max_width(&self) -> Option<usize> {
        self.max_width
    }

    pub(crate) fn set_max_width(&mut self, value: Option<usize>) {
        self.max_width = value;
    }

    pub(crate) fn reflow_text(&self) -> bool {
        self.reflow_text
    }

    pub(crate) fn set_reflow_text(&mut self, value: bool) {
        self.reflow_text = value;
    }

    /// Internal setter for config options. Used for testing
    #[cfg(test)]
    pub(crate) fn set<'a>(
        &mut self,
        field: &'a str,
        value: &str,
    ) -> Result<(), ConfigSetError<'a>> {
        match field {
            "max_width" => {
                let value = value.parse::<usize>().unwrap();
                self.max_width = Some(value)
            }
            "reflow_text" => {
                let value = value.parse::<bool>().unwrap();
                self.reflow_text = value;
            }
            _ => return self.set_unstable(field, value),
        }
        Ok(())
    }
}
