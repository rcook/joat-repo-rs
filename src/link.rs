use crate::link_id::LinkId;
use crate::meta_id::MetaId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub created_at: DateTime<Utc>,
    pub link_id: LinkId,
    pub project_dir: PathBuf,
    pub meta_id: MetaId,
}

#[derive(Debug)]
pub struct LinkEx {
    pub link_path: PathBuf,
    pub link: Link,
}
