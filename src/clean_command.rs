use crate::link::LinkEx;
use crate::manifest::ManifestEx;
use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use log::{error, info};
use std::collections::HashMap;
use std::fs::{remove_dir_all, remove_file};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
struct ManifestStatus {
    manifest: ManifestEx,
    is_referenced: bool,
}

#[derive(Debug)]
struct LinkStatus {
    link: LinkEx,
    is_valid: bool,
}

pub fn do_clean(repo: &Repo) -> Result<Status> {
    let mut manifest_map = repo
        .list_manifests()?
        .into_iter()
        .map(|m| {
            (
                m.manifest.meta_id,
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
        .values()
        .filter(|x| !x.is_valid)
        .collect::<Vec<_>>();
    let invalid_link_count = invalid_links.len();
    if invalid_link_count > 0 {
        println!(
            "The following {} links are invalid and will be removed:",
            invalid_link_count
        );
        for (idx, l) in invalid_links.iter().enumerate() {
            println!("({}) {:#?}", idx + 1, l.link);
        }
    }

    let unreferenced_manifests = manifest_map
        .values()
        .filter(|x| !x.is_referenced)
        .collect::<Vec<_>>();
    let unreferenced_manifest_count = unreferenced_manifests.len();
    if unreferenced_manifest_count > 0 {
        println!(
            "The following {} metadirectories are unreferenced and will be removed:",
            unreferenced_manifest_count
        );
        for (idx, m) in unreferenced_manifests.iter().enumerate() {
            println!("({}) {:#?}", idx + 1, m.manifest);
        }
    }

    if invalid_link_count + unreferenced_manifest_count == 0 {
        info!("No clean-up required");
        return Ok(Status::Success);
    }

    print!("Type DELETE to delete them: ");
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    let line = line.trim().to_lowercase();
    if line != "delete" {
        error!("Aborting clean-up");
        return Ok(Status::Failure);
    }

    for l in invalid_links {
        remove_file(&l.link.link_path)?
    }

    for m in unreferenced_manifests {
        remove_dir_all(&m.manifest.data_dir)?;
    }

    Ok(Status::Success)
}
