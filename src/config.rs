#[derive(Debug, Default)]
pub(crate) struct Config {
    max_width: Option<usize>,
}

impl Config {
    pub(crate) fn max_width(&self) -> Option<usize> {
        self.max_width
    }

    pub(crate) fn set_max_width(&mut self, value: Option<usize>) {
        self.max_width = value;
    }

    /// Internal setter for config options. Used for testing
    #[cfg(test)]
    pub(crate) fn set(&mut self, field: &str, value: &str) {
        match field {
            "max_width" => {
                let value = value.parse::<usize>().unwrap();
                self.max_width = Some(value)
            }
            _ => panic!("unknown configuration {field}"),
        }
    }
}
