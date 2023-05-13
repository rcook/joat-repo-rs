use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetaId(Uuid);

impl MetaId {
    pub fn parse(s: &str) -> Result<Self> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for MetaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_simple())
    }
}

impl Serialize for MetaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0.as_simple()))
    }
}

impl<'de> Deserialize<'de> for MetaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(Uuid::deserialize(deserializer)?))
    }
}
