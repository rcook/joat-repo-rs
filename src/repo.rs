use crate::dir_info::DirInfo;
use crate::link::{Link, LinkEx};
use crate::link_id::LinkId;
use crate::manifest::{Manifest, ManifestEx};
use crate::meta_id::MetaId;
use crate::RepoConfig;
use anyhow::{bail, Result};
use chrono::Utc;
use fslock::LockFile;
use joatmon::{read_yaml_file, safe_write_file, FileReadError, HasOtherError};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const MANIFEST_FILE_NAME: &str = "manifest.yaml";

#[derive(Debug)]
pub struct Repo {
    config: RepoConfig,
    _lock_file: LockFile,
}

impl Repo {
    pub fn new(config: RepoConfig) -> Result<Option<Self>> {
        safe_write_file(&config.lock_path, vec![], true)?;
        let mut lock_file = LockFile::open(&config.lock_path)?;
        Ok(if lock_file.try_lock_with_pid()? {
            Some(Self {
                config,
                _lock_file: lock_file,
            })
        } else {
            None
        })
    }

    pub fn list_links(&self) -> Result<Vec<LinkEx>> {
        let mut links = Vec::new();

        if self.config.links_dir.is_dir() {
            for entry_opt in read_dir(&self.config.links_dir)? {
                let entry = entry_opt?;
                if entry.path().is_file() {
                    if let Some(link) = self.read_link_from_link_path(&entry.path())? {
                        links.push(link)
                    }
                }
            }
        }

        Ok(links)
    }

    pub fn list_manifests(&self) -> Result<Vec<ManifestEx>> {
        let mut manifests = Vec::new();

        if self.config.container_dir.is_dir() {
            for entry_opt in read_dir(&self.config.container_dir)? {
                let entry = entry_opt?;
                if entry.path().is_dir() {
                    manifests.push(self.read_manifest_from_datadir(&entry.path())?);
                }
            }
        }

        Ok(manifests)
    }

    pub fn init(&self, project_dir: &Path) -> Result<Option<DirInfo>> {
        let link_id = LinkId::from_path(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if link_path.is_file() {
            return Ok(None);
        }

        let meta_id = MetaId::random();
        let data_dir = self.make_data_dir(&meta_id);
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);

        let manifest = Manifest {
            created_at: Utc::now(),
            original_project_dir: project_dir.to_path_buf(),
            meta_id: meta_id.clone(),
        };
        safe_write_file(&manifest_path, serde_yaml::to_string(&manifest)?, false)?;

        let link = Link {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id,
        };
        safe_write_file(&link_path, serde_yaml::to_string(&link)?, false)?;

        Ok(Some(DirInfo {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }))
    }

    pub fn get(&self, project_dir: &Path) -> Result<Option<DirInfo>> {
        let link_id = LinkId::from_path(project_dir)?;
        let link_path = self.make_link_path(&link_id);
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

        Ok(Some(DirInfo {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }))
    }

    pub fn read_manifest(&self, meta_id: &MetaId) -> Result<ManifestEx> {
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
        let link_id = LinkId::from_path(project_dir)?;
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

    pub fn link(&self, meta_id: &MetaId, project_dir: &Path) -> Result<Option<DirInfo>> {
        let manifest = self.read_manifest(meta_id)?;

        let link_id = LinkId::from_path(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if link_path.is_file() {
            return Ok(None);
        }

        let link = Link {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id: meta_id.clone(),
        };
        safe_write_file(&link_path, serde_yaml::to_string(&link)?, false)?;

        Ok(Some(DirInfo {
            manifest,
            link: LinkEx { link_path, link },
        }))
    }

    fn make_link_path(&self, link_id: &LinkId) -> PathBuf {
        self.config.links_dir.join(format!("{}.yaml", link_id))
    }

    fn make_data_dir(&self, meta_id: &MetaId) -> PathBuf {
        self.config.container_dir.join(format!("{}", meta_id))
    }
}
