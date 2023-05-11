use crate::repo::Repo;
use anyhow::Result;
use std::fs::read_dir;

pub fn do_list(repo: &Repo) -> Result<()> {
    for entry_opt in read_dir(&repo.dir)? {
        let entry = entry_opt?;
        if entry.path().is_dir() {
            let manifest = repo.read_manifest(&entry.path())?;
            println!("{:#?}", manifest);
        } else if entry.path().is_file() {
            let link = repo.read_link(&entry.path())?;
            println!("{:#?}", link);
        }
    }
    Ok(())
}
