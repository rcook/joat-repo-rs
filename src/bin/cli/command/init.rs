use super::super::Status;
use anyhow::Result;
use faf::Repo;
use log::error;
use std::path::Path;

pub fn do_init(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(if let Some(dir_info) = repo.init(project_dir)? {
        println!("{:#?}", dir_info);
        Status::Success
    } else {
        error!(
            "Directory {} is already in repository",
            project_dir.display()
        );
        Status::Failure
    })
}
