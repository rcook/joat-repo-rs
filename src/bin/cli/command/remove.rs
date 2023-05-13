use super::super::Status;
use anyhow::Result;
use faf::{Repo, Trash};
use log::{error, info};
use std::fs::remove_file;
use std::path::Path;

pub fn do_remove(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(match repo.get_metadir(project_dir)? {
        Some(metadir) => {
            remove_file(metadir.link.link_path)?;
            let trash = Trash::compute(repo)?;
            if !trash.is_empty() {
                info!("There is trash: run \"clean\" command to delete unreferenced files");
            }
            Status::Success
        }
        None => {
            error!(
                "No metadirectory for found for directory {}",
                project_dir.display()
            );
            Status::Failure
        }
    })
}
