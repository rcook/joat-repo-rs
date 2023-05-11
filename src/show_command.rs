use crate::repo::Repo;
use anyhow::Result;
use std::path::Path;

pub fn do_show(repo: &Repo, project_dir: &Path) -> Result<()> {
    match repo.get_metadirectory(project_dir)? {
        Some(metadir) => println!("{:#?}", metadir),
        None => println!("No metadirectory for found for this directory"),
    }
    Ok(())
}
