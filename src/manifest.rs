use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct ManifestEx {
    pub data_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: Manifest,
}
