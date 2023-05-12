use crate::repo::Repo;
use anyhow::Result;
use std::fs::remove_file;
use std::path::Path;

pub fn do_remove(repo: &Repo, project_dir: &Path) -> Result<()> {
    let metadir = match repo.get_metadir(project_dir)? {
        Some(value) => value,
        None => {
            println!("No metadirectory for found for this directory");
            return Ok(());
        }
    };

    remove_file(&metadir.link.link_path)?;
    Ok(())
}
