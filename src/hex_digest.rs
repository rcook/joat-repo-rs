use anyhow::{anyhow, Result};
use md5::compute;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct HexDigest(String);

impl HexDigest {
    pub fn new(value: &str) -> Self {
        Self(String::from(value))
    }

    pub fn from_path(config_path: &Path) -> Result<Self> {
        let config_path_str = config_path.to_str().ok_or(anyhow!("cannot convert path"))?;
        let digest = compute(config_path_str);
        let hex_digest = format!("{:x}", digest);
        Ok(Self(hex_digest))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for HexDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for HexDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(HexDigest::new(&String::deserialize(deserializer)?))
    }
}
