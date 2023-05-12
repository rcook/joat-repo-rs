use crate::repo::Repo;
use anyhow::Result;
use std::path::Path;
use uuid::Uuid;

pub fn do_link(repo: &Repo, meta_id: &Option<Uuid>, project_dir: &Path) -> Result<()> {
    let m = meta_id.as_ref().expect("");
    let manifest = repo.read_manifest(m)?;
    println!("manifest={:#?}", manifest);

    //repo.link_metadir(reference, project_dir)?;
    Ok(())
}
