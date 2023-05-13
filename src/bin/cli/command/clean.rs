use super::super::{prompt, Status};
use anyhow::Result;
use faf::{Repo, Trash};
use log::{error, info};
use std::fs::{remove_dir_all, remove_file};

pub fn do_clean(repo: &Repo, force: bool) -> Result<Status> {
    let trash = Trash::compute(repo)?;
    if trash.is_empty() {
        info!("No clean-up required");
        return Ok(Status::Success);
    }

    let invalid_link_count = trash.invalid_links.len();
    if invalid_link_count > 0 {
        println!(
            "The following {} links are invalid and will be removed:",
            invalid_link_count
        );
        for (idx, l) in trash.invalid_links.iter().enumerate() {
            println!("({}) {:#?}", idx + 1, l);
        }
    }

    let unreferenced_manifest_count = trash.unreferenced_manifests.len();
    if unreferenced_manifest_count > 0 {
        println!(
            "The following {} metadirectories are unreferenced and will be removed:",
            unreferenced_manifest_count
        );
        for (idx, m) in trash.unreferenced_manifests.iter().enumerate() {
            println!("({}) {:#?}", idx + 1, m);
        }
    }

    if !force && prompt("Type DELETE to delete them")? != "delete" {
        error!("Aborting clean-up");
        return Ok(Status::Failure);
    }

    for l in trash.invalid_links {
        remove_file(&l.link_path)?
    }

    for m in trash.unreferenced_manifests {
        remove_dir_all(&m.data_dir)?;
    }

    Ok(Status::Success)
}
