use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;

pub fn do_list(repo: &Repo) -> Result<Status> {
    for manifest in repo.list_manifests()? {
        println!("{:#?}", manifest);
    }

    for link in repo.list_links()? {
        println!("{:#?}", link);
    }

    Ok(Status::Success)
}
