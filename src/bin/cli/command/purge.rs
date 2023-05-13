use super::super::Status;
use anyhow::Result;
use joat_repo::Repo;
use log::error;

pub fn do_purge(repo: &Repo, force: bool) -> Result<Status> {
    if !force {
        error!("This operation will delete all of your metadirectories: pass --force if you're sure what you're doing");
        return Ok(Status::Failure);
    }

    repo.purge()?;
    Ok(Status::Success)
}
