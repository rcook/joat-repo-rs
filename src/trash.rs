use crate::link::LinkEx;
use crate::manifest::ManifestEx;
use crate::repo::Repo;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::{remove_dir_all, remove_file};

#[derive(Debug)]
pub struct Trash {
    pub unreferenced_manifests: Vec<ManifestEx>,
    pub invalid_links: Vec<LinkEx>,
}

struct ManifestStatus {
    manifest: ManifestEx,
    is_referenced: bool,
}

struct LinkStatus {
    link: LinkEx,
    is_valid: bool,
}

impl Trash {
    pub fn compute(repo: &Repo) -> Result<Self> {
        let mut manifest_map = repo
            .list_manifests()?
            .into_iter()
            .map(|m| {
                (
                    m.manifest.meta_id.clone(),
                    ManifestStatus {
                        manifest: m,
                        is_referenced: false,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let mut link_map = repo
            .list_links()?
            .into_iter()
            .map(|l| {
                (
                    l.link.link_id.clone(),
                    LinkStatus {
                        link: l,
                        is_valid: true,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        for l in link_map.values_mut() {
            if l.link.link.project_dir.is_dir() {
                match manifest_map.get_mut(&l.link.link.meta_id) {
                    Some(m) => m.is_referenced = true,
                    None => l.is_valid = false,
                }
            } else {
                l.is_valid = false
            }
        }

        let invalid_links = link_map
            .into_values()
            .filter(|x| !x.is_valid)
            .map(|x| x.link)
            .collect::<Vec<_>>();
        let unreferenced_manifests = manifest_map
            .into_values()
            .filter(|x| !x.is_referenced)
            .map(|x| x.manifest)
            .collect::<Vec<_>>();

        Ok(Self {
            invalid_links,
            unreferenced_manifests,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.invalid_links.len() + self.unreferenced_manifests.len() == 0
    }

    pub fn empty(&mut self) -> Result<()> {
        for l in self.invalid_links.drain(..) {
            remove_file(&l.link_path)?
        }

        for m in self.unreferenced_manifests.drain(..) {
            remove_dir_all(&m.data_dir)?;
        }

        Ok(())
    }
}
