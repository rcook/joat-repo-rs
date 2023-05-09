use crate::dir::Dir;
use crate::repo::Repo;
use anyhow::Result;

pub fn do_init(repo: &Repo, dir: &Dir) -> Result<()> {
    repo.init_metadata(dir)?;
    Ok(())
}
