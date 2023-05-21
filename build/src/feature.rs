#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub(crate) struct Feature {
    pub name: String,
}

impl Feature {
    pub fn name(&self) -> &str {
        &self.name
    }
}
