use anyhow::{anyhow, Result};
use md5::compute;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, path::Path};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LinkId(String);

impl LinkId {
    pub fn new(s: &str) -> Self {
        Self(String::from(s))
    }

    pub fn from_path(project_dir: &Path) -> Result<Self> {
        assert!(project_dir.is_absolute());
        let project_dir_str = project_dir.to_str().ok_or(anyhow!("cannot convert path"))?;
        let digest = compute(project_dir_str);
        let hex_digest = format!("{:x}", digest);
        Ok(Self(hex_digest))
    }
}

impl Display for LinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for LinkId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for LinkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(&String::deserialize(deserializer)?))
    }
}
