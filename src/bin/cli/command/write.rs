use super::super::Status;
use anyhow::Result;
use joat_repo::{Repo, SharedPath};

pub fn do_write(repo: &Repo, path: &SharedPath, value: &str) -> Result<Status> {
    repo.write_shared_file(path, value)?;
    Ok(Status::Success)
}
