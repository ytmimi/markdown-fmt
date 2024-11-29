/// A buffer for writing Definition lists
#[derive(Debug, PartialEq)]
pub(super) struct DefinitionListTitle {
    buffer: String,
}

impl std::fmt::Write for DefinitionListTitle {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }
}

impl DefinitionListTitle {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub(super) fn into_buffer(self) -> String {
        self.buffer
    }
}
