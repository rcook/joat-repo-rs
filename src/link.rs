use crate::hex_digest::HexDigest;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub id: HexDigest,
    pub project_dir: PathBuf,
    pub reference: Uuid,
}

#[derive(Debug)]
pub struct LinkEx {
    pub link_path: PathBuf,
    pub link: Link,
}
