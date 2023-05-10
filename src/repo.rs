use crate::dir::Dir;
use crate::hash::HexDigest;
use crate::manifest::Manifest;
use crate::metadata::Metadata;
use anyhow::Result;
use joatmon::{read_yaml_file, safe_write_file};
use std::iter::once;
use std::path::{Path, PathBuf};

const MANIFEST_FILE_NAME: &str = "manifest.yaml";

pub struct Repo {
    pub dir: PathBuf,
}

impl Repo {
    pub fn new(dir: &Path) -> Self {
        Self {
            dir: PathBuf::from(dir),
        }
    }

    pub fn get_metadata_from_data_dir(&self, data_dir: &Path) -> Result<Metadata> {
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file(&manifest_path)?;
        Ok(Metadata {
            data_dir: data_dir.to_path_buf(),
            manifest_path,
            manifest,
        })
    }

    pub fn get_metadata(&self, dir: &Dir) -> Result<Option<Metadata>> {
        for dir_path in once(&dir.path).chain(&dir.other_paths) {
            let digest = HexDigest::from_path(dir_path)?;
            let data_dir = self.dir.join(digest.as_str());
            let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
            if manifest_path.is_file() {
                let manifest = read_yaml_file::<Manifest, _>(&manifest_path)?;
                if manifest.path == *dir_path || manifest.other_paths.contains(dir_path) {
                    return Ok(Some(Metadata {
                        data_dir,
                        manifest_path: manifest_path,
                        manifest,
                    }));
                }
            }
        }
        Ok(None)
    }

    pub fn init_metadata(&self, dir: &Dir) -> Result<()> {
        let digest = HexDigest::from_path(&dir.path)?;
        let data_dir = self.dir.join(digest.as_str());
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = Manifest {
            path: dir.path.clone(),
            other_paths: dir.other_paths.clone(),
        };
        safe_write_file(manifest_path, serde_yaml::to_string(&manifest)?, false)?;
        Ok(())
    }
}
