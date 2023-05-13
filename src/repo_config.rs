use crate::repo::Repo;
use anyhow::Result;
use joatmon::{read_yaml_file, safe_write_file};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoConfig {
    pub lock_path: PathBuf,
    pub config_path: PathBuf,
    pub links_dir: PathBuf,
    pub container_dir: PathBuf,
    pub shared_dir: PathBuf,
}

impl RepoConfig {
    pub fn default(base_dir: &Path, prefix: Option<&str>) -> Self {
        let full_prefix = prefix.map(|s| format!("{}-", s)).unwrap_or_default();
        Self {
            lock_path: base_dir.join(format!(".{}lock", full_prefix)),
            config_path: base_dir.join(format!("{}config.yaml", full_prefix)),
            links_dir: base_dir.join(format!("{}links", full_prefix)),
            container_dir: base_dir.join(format!("{}data", full_prefix)),
            shared_dir: base_dir.join(format!("{}shared", full_prefix)),
        }
    }

    pub fn repo(self) -> Result<Option<Repo>> {
        Repo::new(if self.config_path.is_file() {
            read_yaml_file::<RepoConfig, _>(&self.config_path)?
        } else {
            safe_write_file(&self.config_path, serde_yaml::to_string(&self)?, false)?;
            self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tempdir::TempDir;

    #[test]
    fn default() -> Result<()> {
        let base_dir = TempDir::new("faf-test")?;
        let c = RepoConfig::default(base_dir.path(), None);
        assert_eq!(base_dir.path().join(".lock"), c.lock_path);
        assert_eq!(base_dir.path().join("config.yaml"), c.config_path);
        assert_eq!(base_dir.path().join("links"), c.links_dir);
        assert_eq!(base_dir.path().join("data"), c.container_dir);
        assert_eq!(base_dir.path().join("shared"), c.shared_dir);
        Ok(())
    }

    #[test]
    fn prefix() -> Result<()> {
        let base_dir = TempDir::new("faf-test")?;
        let c = RepoConfig::default(base_dir.path(), Some("foo"));
        assert_eq!(base_dir.path().join(".foo-lock"), c.lock_path);
        assert_eq!(base_dir.path().join("foo-config.yaml"), c.config_path);
        assert_eq!(base_dir.path().join("foo-links"), c.links_dir);
        assert_eq!(base_dir.path().join("foo-data"), c.container_dir);
        assert_eq!(base_dir.path().join("foo-shared"), c.shared_dir);
        Ok(())
    }
}
