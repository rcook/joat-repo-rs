use crate::repo::Repo;
use anyhow::Result;
use std::path::Path;

pub fn do_init(repo: &Repo, project_dir: &Path) -> Result<()> {
    let metadir = repo.init_metadir(project_dir)?;
    println!("{:#?}", metadir);
    Ok(())
}
