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
}
