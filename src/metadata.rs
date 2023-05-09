use crate::manifest::Manifest;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Metadata {
    pub data_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: Manifest,
}
