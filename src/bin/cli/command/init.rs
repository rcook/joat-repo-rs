use super::super::Status;
use anyhow::Result;
use faf::Repo;
use log::error;
use std::path::Path;

pub fn do_init(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(if let Some(metadir) = repo.init_metadir(project_dir)? {
        println!("{:#?}", metadir);
        Status::Success
    } else {
        error!(
            "Link already exists for directory {}",
            project_dir.display()
        );
        Status::Failure
    })
}
