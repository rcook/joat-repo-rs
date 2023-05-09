use anyhow::{anyhow, Result};
use md5::compute;
use std::path::Path;

#[derive(Debug)]
pub struct HexDigest(String);

impl HexDigest {
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
