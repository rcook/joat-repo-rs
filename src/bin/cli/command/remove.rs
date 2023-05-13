use super::super::Status;
use anyhow::Result;
use joat_repo::{Repo, Trash};
use log::error;
use std::fs::remove_file;
use std::path::Path;

pub fn do_remove(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(match repo.get(project_dir)? {
        Some(dir_info) => {
            remove_file(dir_info.link.link_path)?;
            Trash::compute(repo)?.empty()?;
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
