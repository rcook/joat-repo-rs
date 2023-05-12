use crate::repo::Repo;
use anyhow::Result;

pub fn do_list(repo: &Repo) -> Result<()> {
    for manifest in repo.list_manifests()? {
        println!("{:#?}", manifest);
    }

    for link in repo.list_links()? {
        println!("{:#?}", link);
    }

    Ok(())
}
