use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub created_at: DateTime<Utc>,
    pub meta_id: Uuid,
}

#[derive(Debug)]
pub struct ManifestEx {
    pub data_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: Manifest,
}