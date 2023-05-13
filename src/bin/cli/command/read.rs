use super::super::Status;
use anyhow::Result;
use joat_repo::{Repo, SharedPath};
use log::error;

pub fn do_read(repo: &Repo, path: &SharedPath) -> Result<Status> {
    match repo.read_shared_file(path)? {
        Some(s) => {
            println!("{}", s);
            Ok(Status::Success)
        }
        None => {
            error!("Shared file {} not found", path);
            Ok(Status::Failure)
        }
    }
}
