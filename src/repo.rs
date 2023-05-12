use crate::hex_digest::HexDigest;
use crate::link::{Link, LinkEx};
use crate::manifest::{Manifest, ManifestEx};
use crate::metadir::Metadir;
use anyhow::{bail, Result};
use chrono::Utc;
use joatmon::{read_yaml_file, safe_write_file, FileReadError, HasOtherError};
use log::info;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use uuid::Uuid;

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

    pub fn list_manifests(&self) -> Result<Vec<ManifestEx>> {
        let mut manifests = Vec::new();

        for entry_opt in read_dir(&self.dir)? {
            let entry = entry_opt?;
            if entry.path().is_dir() {
                manifests.push(self.read_manifest_from_datadir(&entry.path())?);
            }
        }

        Ok(manifests)
    }

    pub fn list_links(&self) -> Result<Vec<LinkEx>> {
        let mut links = Vec::new();

        for entry_opt in read_dir(&self.dir)? {
            let entry = entry_opt?;
            if entry.path().is_file() {
                if let Some(link) = self.read_link_from_link_path(&entry.path())? {
                    links.push(link)
                }
            }
        }

        Ok(links)
    }

    pub fn init_metadir(&self, project_dir: &Path) -> Result<Option<Metadir>> {
        let link_id = HexDigest::from_path(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if link_path.is_file() {
            info!(
                "link file {} already exists for directory {}",
                link_path.display(),
                project_dir.display()
            );
            return Ok(None);
        }

        let meta_id = Uuid::new_v4();
        let data_dir = self.make_data_dir(&meta_id);
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);

        let manifest = Manifest {
            created_at: Utc::now(),
            meta_id,
        };
        safe_write_file(&manifest_path, serde_yaml::to_string(&manifest)?, false)?;

        let link = Link {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id,
        };
        safe_write_file(&link_path, serde_yaml::to_string(&link)?, false)?;

        Ok(Some(Metadir {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }))
    }

    pub fn get_metadir(&self, project_dir: &Path) -> Result<Option<Metadir>> {
        let link_id = HexDigest::from_path(project_dir)?;
        let link_path = self.dir.join(format!("{}.yaml", link_id.as_str()));
        if !link_path.is_file() {
            return Ok(None);
        }

        let link = read_yaml_file::<Link, _>(&link_path)?;
        if link.project_dir != *project_dir {
            bail!(
                "link file {} project directory {} does not match expected directory {}",
                link_path.display(),
                link.project_dir.display(),
                project_dir.display()
            )
        }

        let data_dir = self.make_data_dir(&link.meta_id);
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file::<Manifest, _>(&manifest_path)?;

        Ok(Some(Metadir {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }))
    }

    pub fn read_manifest(&self, meta_id: &Uuid) -> Result<ManifestEx> {
        let manifest_path = self.make_data_dir(meta_id);
        self.read_manifest_from_datadir(&manifest_path)
    }

    pub fn read_manifest_from_datadir(&self, data_dir: &Path) -> Result<ManifestEx> {
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file(&manifest_path)?;
        Ok(ManifestEx {
            data_dir: data_dir.to_path_buf(),
            manifest_path,
            manifest,
        })
    }

    pub fn read_link(&self, project_dir: &Path) -> Result<Option<LinkEx>> {
        let link_id = HexDigest::from_path(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        self.read_link_from_link_path(&link_path)
    }

    pub fn read_link_from_link_path(&self, link_path: &Path) -> Result<Option<LinkEx>> {
        match read_yaml_file(link_path) {
            Ok(link) => Ok(Some(LinkEx {
                link_path: link_path.to_path_buf(),
                link,
            })),
            Err(e)
                if e.downcast_other_ref::<FileReadError>()
                    .map(FileReadError::is_not_found)
                    .unwrap_or(false) =>
            {
                Ok(None)
            }
            Err(e) => bail!(e),
        }
    }

    pub fn link_metadir(&self, meta_id: &Uuid, project_dir: &Path) -> Result<Option<Metadir>> {
        let manifest = self.read_manifest(meta_id)?;

        let link_id = HexDigest::from_path(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if link_path.is_file() {
            return Ok(None);
        }

        let link = Link {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id: *meta_id,
        };
        safe_write_file(&link_path, serde_yaml::to_string(&link)?, false)?;

        Ok(Some(Metadir {
            manifest,
            link: LinkEx { link_path, link },
        }))
    }

    fn make_link_path(&self, link_id: &HexDigest) -> PathBuf {
        self.dir.join(format!("{}.yaml", link_id.as_str()))
    }

    fn make_data_dir(&self, meta_id: &Uuid) -> PathBuf {
        self.dir.join(format!("{}", meta_id.as_simple()))
    }
}
