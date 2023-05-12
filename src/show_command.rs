use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use std::path::Path;

pub fn do_show(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(match repo.get_metadir(project_dir)? {
        Some(metadir) => {
            println!("{:#?}", metadir);
            Status::Success
        }
        None => {
            println!(
                "No metadirectory for found for directory {}",
                project_dir.display()
            );
            Status::Failure
        }
    })
}
