use crate::repo::Repo;
use anyhow::Result;
use std::path::Path;

pub fn do_register(repo: &Repo, hash: &str, dir: &Path) -> Result<()> {
    let data_dir = repo.dir.join(hash);
    let mut metadata = repo.get_metadata_from_data_dir(&data_dir)?;
    _ = metadata.manifest.other_paths.insert(dir.to_path_buf());
    repo.write_metadata(&metadata, true)?;
    Ok(())
}
