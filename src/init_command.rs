use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use std::path::Path;

pub fn do_init(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(if let Some(metadir) = repo.init_metadir(project_dir)? {
        println!("{:#?}", metadir);
        Status::Success
    } else {
        Status::Failure
    })
}
