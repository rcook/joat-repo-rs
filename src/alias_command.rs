use crate::hex_digest::HexDigest;
use crate::repo::Repo;
use anyhow::Result;
use std::path::Path;

pub fn do_alias(repo: &Repo, reference: &HexDigest, project_dir: &Path) -> Result<()> {
    repo.init_metadata_alias(reference, project_dir)?;
    Ok(())
}
