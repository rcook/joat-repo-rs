use super::super::Status;
use anyhow::Result;
use faf::Repo;
use log::{error, info};
use std::path::Path;

pub fn do_show(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(match repo.get(project_dir)? {
        Some(dir_info) => {
            info!("{:#?}", dir_info);
            Status::Success
        }
        None => {
            error!(
                "No metadirectory found for directory {}",
                project_dir.display()
            );
            Status::Failure
        }
    })
}
