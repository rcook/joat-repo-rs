use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SharedPath(String);

impl SharedPath {
    pub fn new(s: &str) -> Self {
        Self(String::from(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SharedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
