//! Where all unstable features are exposed

use crate::FormatBuilder;
#[allow(unused_imports)]
use crate::config::Config;
use crate::config::options::UnorderedListMarkerConfig;

#[cfg(test)]
macro_rules! set_unstable_config {
    (if feature = $name:literal $body:tt) => {
        if cfg!(any(
            feature = "unstable-configs",
            feature = $name
        )) {
            $body
            Ok(())
        } else {
            Err(crate::config::ConfigSetError::MissingFeature($name))
        }
    };
}

macro_rules! get_unstable_config {
    (if feature = $name:literal $body:tt) => {
        #[cfg(any(feature = "unstable-configs", feature = $name))]
        {
            $body
        }
        #[cfg(not(any(feature = "unstable-configs", feature = $name)))]
        None
    };
}

impl Config {
    /// Internal setter for unstable config options. Used for testing.
    #[cfg(test)]
    pub(crate) fn set_unstable<'a>(
        &mut self,
        field: &'a str,
        _value: &str,
    ) -> Result<(), crate::config::ConfigSetError<'a>> {
        match field {
            "unordered_list_marker" => {
                set_unstable_config! {
                    if feature = "unordered-list-marker" {
                        let value = _value.parse::<UnorderedListMarkerConfig>().unwrap();
                        self.set_unordered_list_marker(Some(value));
                    }
                }
            }
            _ => {
                return Err(crate::config::ConfigSetError::UnknownConfig(field));
            }
        }
    }

    pub(crate) fn unordered_list_marker(&self) -> Option<UnorderedListMarkerConfig> {
        get_unstable_config! {
            if feature = "unordered-list-marker" {
                self.unordered_list_marker
            }
        }
    }

    // Don't want this warning about dead code when the required features aren't set.
    #[allow(dead_code)]
    pub(crate) fn set_unordered_list_marker(&mut self, _value: Option<UnorderedListMarkerConfig>) {
        #[cfg(any(feature = "unstable-configs", feature = "unordered-list-marker"))]
        {
            self.unordered_list_marker = _value;
        }
    }
}

impl FormatBuilder {
    /// Normalize all unorderd lists to use the same marker.
    ///
    /// When set to [None], the default, list markers are inferred from the source code.
    ///
    /// # Setting [`unordered_list_marker`](Self::unordered_list_marker) to [None] (default)
    ///
    /// ```rust
    /// # use markdown_fmt::FormatBuilder;
    /// let mut builder = FormatBuilder::default();
    /// builder.unordered_list_marker(None);
    ///
    /// let input = "* list with a `*` marker";
    /// let output = builder.build().format(input).unwrap();
    /// assert_eq!(output, input)
    /// ```
    /// ---
    ///
    /// # Setting [`unordered_list_marker`](Self::unordered_list_marker) to [Asterisk]
    ///
    /// [Asterisk]: crate::config::options::UnorderedListMarkerConfig::Asterisk
    ///
    /// ```rust
    /// # use markdown_fmt::FormatBuilder;
    /// use markdown_fmt::options::UnorderedListMarkerConfig;
    /// let mut builder = FormatBuilder::default();
    /// builder.unordered_list_marker(Some(UnorderedListMarkerConfig::Asterisk));
    ///
    /// let input = "+ list with a `+` marker";
    ///
    /// let expected = "* list with a `+` marker";
    /// let output = builder.build().format(input).unwrap();
    /// assert_eq!(output, expected)
    /// ```
    #[cfg(any(feature = "unstable-configs", feature = "unordered-list-marker"))]
    pub fn unordered_list_marker(&mut self, value: Option<UnorderedListMarkerConfig>) -> &mut Self {
        self.config.set_unordered_list_marker(value);
        self
    }
}
