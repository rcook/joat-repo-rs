use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub path: PathBuf,
    pub other_paths: HashSet<PathBuf>,
}
