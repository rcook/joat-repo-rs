use super::super::Status;
use anyhow::Result;
use faf::Repo;

pub fn do_info(repo: &Repo) -> Result<Status> {
    println!("{:#?}", repo);
    Ok(Status::Success)
}
