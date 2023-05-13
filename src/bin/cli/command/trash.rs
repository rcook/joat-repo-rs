use super::super::Status;
use anyhow::Result;
use joat_repo::{Repo, Trash};
use log::info;

pub fn do_trash(repo: &Repo, clean: bool) -> Result<Status> {
    let mut trash = Trash::compute(repo)?;

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

    if clean {
        trash.empty()?;
    }

    Ok(Status::Success)
}
