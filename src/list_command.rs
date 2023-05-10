use crate::repo::Repo;
use anyhow::Result;
use std::fs::read_dir;

pub fn do_list(repo: &Repo) -> Result<()> {
    for entry_opt in read_dir(&repo.dir)? {
        let entry = entry_opt?;
        let metadata = repo.get_metadata_from_data_dir(&entry.path())?;
        println!("metadata={:#?}", metadata);
    }
    Ok(())
}
