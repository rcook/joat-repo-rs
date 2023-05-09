use crate::dir::Dir;
use crate::repo::Repo;
use anyhow::Result;
use std::fs::remove_dir_all;

pub fn do_remove(repo: &Repo, dir: &Dir) -> Result<()> {
    match repo.get_metadata(dir)? {
        Some(metadata) => remove_dir_all(metadata.data_dir)?,
        None => println!("No metadata for found for this directory"),
    }
    Ok(())
}
