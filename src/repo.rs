// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::dir_info::DirInfo;
use crate::link::{Link, LinkEx};
use crate::link_id::LinkId;
use crate::manifest::{Manifest, ManifestEx};
use crate::meta_id::MetaId;
use crate::repo_error::RepoError;
use crate::repo_result::RepoResult;
use crate::shared_path::SharedPath;
use crate::RepoConfig;
use chrono::Utc;
use fslock::LockFile;
use joatmon::{read_text_file, read_yaml_file, safe_write_file, FileReadError, HasOtherError};
use path_absolutize::Absolutize;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::{Path, PathBuf};

const MANIFEST_FILE_NAME: &str = "manifest.yaml";

#[derive(Debug)]
pub struct Repo {
    config: RepoConfig,
    _lock_file: LockFile,
}

impl Repo {
    pub fn new(config: RepoConfig) -> RepoResult<Option<Self>> {
        safe_write_file(&config.lock_path, vec![], true).map_err(RepoError::other)?;
        let mut lock_file = LockFile::open(&config.lock_path)
            .map_err(|_e| RepoError::could_not_open_lock_file(&config.lock_path))?;
        Ok(
            if lock_file
                .try_lock_with_pid()
                .map_err(|_e| RepoError::could_not_lock(&config.lock_path))?
            {
                Some(Self {
                    config,
                    _lock_file: lock_file,
                })
            } else {
                None
            },
        )
    }

    pub fn list_links(&self) -> RepoResult<Vec<LinkEx>> {
        let mut links = Vec::new();

        if self.config.links_dir.is_dir() {
            for entry_opt in read_dir(&self.config.links_dir).map_err(RepoError::other)? {
                let entry = entry_opt.map_err(RepoError::other)?;
                if entry.path().is_file() {
                    if let Some(link) = self.read_link_from_link_path(&entry.path())? {
                        links.push(link)
                    }
                }
            }
        }

        Ok(links)
    }

    pub fn list_manifests(&self) -> RepoResult<Vec<ManifestEx>> {
        let mut manifests = Vec::new();

        if self.config.container_dir.is_dir() {
            for entry_opt in read_dir(&self.config.container_dir).map_err(RepoError::other)? {
                let entry = entry_opt.map_err(RepoError::other)?;
                if entry.path().is_dir() {
                    manifests.push(self.read_manifest_from_datadir(&entry.path())?);
                }
            }
        }

        Ok(manifests)
    }

    pub fn init(&self, project_dir: &Path) -> RepoResult<Option<DirInfo>> {
        let link_id = Self::make_link_id(project_dir)?;
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
        let yaml_str = serde_yaml::to_string(&manifest).map_err(RepoError::other)?;
        safe_write_file(&manifest_path, yaml_str, false).map_err(RepoError::other)?;

        let link = Link {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id,
        };
        let yaml_str = serde_yaml::to_string(&link).map_err(RepoError::other)?;
        safe_write_file(&link_path, yaml_str, false).map_err(RepoError::other)?;

        Ok(Some(DirInfo {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }))
    }

    pub fn get(&self, project_dir: &Path) -> RepoResult<Option<DirInfo>> {
        let link_id = Self::make_link_id(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if !link_path.is_file() {
            return Ok(None);
        }

        let link = read_yaml_file::<Link, _>(&link_path).map_err(RepoError::other)?;
        if link.project_dir != *project_dir {
            return Err(RepoError::invalid_link_file(
                &link_path,
                &link.project_dir,
                project_dir,
            ));
        }

        let data_dir = self.make_data_dir(&link.meta_id);
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file::<Manifest, _>(&manifest_path).map_err(RepoError::other)?;

        Ok(Some(DirInfo {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }))
    }

    pub fn read_manifest(&self, meta_id: &MetaId) -> RepoResult<ManifestEx> {
        let manifest_path = self.make_data_dir(meta_id);
        self.read_manifest_from_datadir(&manifest_path)
    }

    pub fn read_manifest_from_datadir(&self, data_dir: &Path) -> RepoResult<ManifestEx> {
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file(&manifest_path).map_err(RepoError::other)?;
        Ok(ManifestEx {
            data_dir: data_dir.to_path_buf(),
            manifest_path,
            manifest,
        })
    }

    pub fn read_link(&self, project_dir: &Path) -> RepoResult<Option<LinkEx>> {
        let link_id = Self::make_link_id(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        self.read_link_from_link_path(&link_path)
    }

    pub fn read_link_from_link_path(&self, link_path: &Path) -> RepoResult<Option<LinkEx>> {
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
            Err(e) => Err(RepoError::other(e)),
        }
    }

    pub fn link(&self, meta_id: &MetaId, project_dir: &Path) -> RepoResult<Option<DirInfo>> {
        let manifest = self.read_manifest(meta_id)?;

        let link_id = Self::make_link_id(project_dir)?;
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
        let yaml_str = serde_yaml::to_string(&link).map_err(RepoError::other)?;
        safe_write_file(&link_path, yaml_str, false).map_err(RepoError::other)?;

        Ok(Some(DirInfo {
            manifest,
            link: LinkEx { link_path, link },
        }))
    }

    pub fn purge(&self) -> RepoResult<()> {
        if self.config.shared_dir.is_dir() {
            remove_dir_all(&self.config.shared_dir)
                .map_err(|_e| RepoError::could_not_delete_directory(&self.config.shared_dir))?;
        }
        if self.config.container_dir.is_dir() {
            remove_dir_all(&self.config.container_dir)
                .map_err(|_e| RepoError::could_not_delete_directory(&self.config.container_dir))?;
        }
        if self.config.links_dir.is_dir() {
            remove_dir_all(&self.config.links_dir)
                .map_err(|_e| RepoError::could_not_delete_directory(&self.config.links_dir))?;
        }
        if self.config.config_path.is_file() {
            remove_file(&self.config.config_path)
                .map_err(|_e| RepoError::could_not_delete_file(&self.config.config_path))?;
        }
        if self.config.lock_path.is_file() {
            remove_file(&self.config.lock_path)
                .map_err(|_e| RepoError::could_not_delete_file(&self.config.lock_path))?;
        }
        Ok(())
    }

    pub fn read_shared_file(&self, path: &SharedPath) -> RepoResult<Option<String>> {
        let p = self.resolve_shared_path(path)?;
        Ok(match read_text_file(p) {
            Ok(s) => Some(s),
            Err(e) if e.is_not_found() => None,
            Err(e) => return Err(RepoError::other(e)),
        })
    }

    pub fn write_shared_file(&self, path: &SharedPath, value: &str) -> RepoResult<()> {
        let p = self.resolve_shared_path(path)?;
        safe_write_file(p, value, true).map_err(RepoError::other)?;
        Ok(())
    }

    fn make_link_id(project_dir: &Path) -> RepoResult<LinkId> {
        LinkId::from_path(project_dir).ok_or(RepoError::could_not_compute_hash(project_dir))
    }

    fn make_link_path(&self, link_id: &LinkId) -> PathBuf {
        self.config.links_dir.join(format!("{}.yaml", link_id))
    }

    fn make_data_dir(&self, meta_id: &MetaId) -> PathBuf {
        self.config.container_dir.join(format!("{}", meta_id))
    }

    fn resolve_shared_path(&self, path: &SharedPath) -> RepoResult<PathBuf> {
        let p = Path::new(path.as_str())
            .absolutize_from(&self.config.shared_dir)
            .map_err(RepoError::other)?
            .into_owned();
        if !p.starts_with(&self.config.shared_dir) {
            return Err(RepoError::invalid_shared_path(path));
        }
        Ok(p)
    }
}
