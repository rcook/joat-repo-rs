use crate::hex_digest::HexDigest;
use crate::link::{Link, LinkEx};
use crate::manifest::{Manifest, ManifestEx};
use crate::metadir::Metadir;
use anyhow::{bail, Result};
use joatmon::{read_yaml_file, safe_write_file};
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

    pub fn init_metadir(&self, project_dir: &Path) -> Result<Metadir> {
        let link_id = HexDigest::from_path(project_dir)?;
        let link_path = self.dir.join(format!("{}.yaml", link_id.as_str()));
        if link_path.is_file() {
            bail!(
                "link file {} already exists for directory {}",
                link_path.display(),
                project_dir.display()
            );
        }

        let id = Uuid::new_v4();
        let data_dir = self.dir.join(format!("{}", id));
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);

        let manifest = Manifest { id };
        safe_write_file(&manifest_path, serde_yaml::to_string(&manifest)?, false)?;

        let link = Link {
            id: link_id,
            project_dir: project_dir.to_path_buf(),
            reference: id,
        };
        safe_write_file(&link_path, serde_yaml::to_string(&link)?, false)?;

        return Ok(Metadir {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        });
    }

    pub fn get_metadir(&self, project_dir: &Path) -> Result<Option<Metadir>> {
        let link_id = HexDigest::from_path(project_dir)?;
        let link_path = self.dir.join(format!("{}.yaml", link_id.as_str()));
        if !link_path.is_file() {
            bail!(
                "link file does not exit for directory {}",
                project_dir.display()
            );
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

        let data_dir = self.dir.join(format!("{}", link.reference));
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file::<Manifest, _>(&manifest_path)?;

        return Ok(Some(Metadir {
            manifest: ManifestEx {
                data_dir,
                manifest_path,
                manifest,
            },
            link: LinkEx { link_path, link },
        }));
    }

    pub fn read_manifest(&self, data_dir: &Path) -> Result<ManifestEx> {
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest = read_yaml_file(&manifest_path)?;
        Ok(ManifestEx {
            data_dir: data_dir.to_path_buf(),
            manifest_path,
            manifest,
        })
    }

    pub fn read_link(&self, link_path: &Path) -> Result<LinkEx> {
        let link = read_yaml_file(&link_path)?;
        Ok(LinkEx {
            link_path: link_path.to_path_buf(),
            link,
        })
    }

    pub fn init_metadata_alias(&self, reference: &HexDigest, project_dir: &Path) -> Result<()> {
        todo!();
        /*
        let id = HexDigest::from_path(project_dir)?;

        let alias = Alias {
            id: id.clone(),
            project_dir: project_dir.to_path_buf(),
            reference: reference.clone(),
        };

        let alias_path = self.dir.join(format!("{}.yaml", id.as_str()));
        safe_write_file(alias_path, serde_yaml::to_string(&alias)?, false)?;
        Ok(())
        */
    }

    #[allow(unused)]
    pub fn write_metadata(&self, metadir: &Metadir, overwrite: bool) -> Result<()> {
        todo!();
        /*
        safe_write_file(
            &metadata.manifest_path,
            serde_yaml::to_string(&metadata.manifest)?,
            overwrite,
        )?;
        Ok(())
        */
    }
}
