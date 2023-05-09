use crate::dir::Dir;
use crate::repo::Repo;
use anyhow::Result;

pub fn do_show(repo: &Repo, dir: &Dir) -> Result<()> {
    match repo.get_metadata(dir)? {
        Some(metadata) => println!("metadata={:#?}", metadata),
        None => println!("No metadata for found for this directory"),
    }
    Ok(())
}
