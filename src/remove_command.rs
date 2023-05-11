use crate::repo::Repo;
use anyhow::Result;
use std::fs::remove_dir_all;
use std::path::Path;

pub fn do_remove(repo: &Repo, project_dir: &Path) -> Result<()> {
    /*
    match repo.get_metadata(project_dir)? {
        Some(metadata) => remove_dir_all(metadata.data_dir)?,
        None => println!("No metadata for found for this directory"),
    }
    */
    Ok(())
}
