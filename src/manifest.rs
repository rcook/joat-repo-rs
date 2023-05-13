use crate::meta_id::MetaId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub created_at: DateTime<Utc>,
    pub original_project_dir: PathBuf,
    pub meta_id: MetaId,
}

#[derive(Clone, Debug)]
pub struct ManifestEx {
    pub data_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: Manifest,
}
