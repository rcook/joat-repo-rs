use anyhow::{bail, Result};
use std::collections::HashSet;
use std::env::{current_dir, var, VarError};
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Dir {
    pub path: PathBuf,
    pub other_paths: HashSet<PathBuf>,
}

impl Dir {
    pub fn from_cwd() -> Result<Self> {
        let path = current_dir()?;
        let mut other_paths = HashSet::new();
        if let Some(pwd) = Self::var_opt("PWD")? {
            let pwd_path = PathBuf::from(pwd);
            if pwd_path != path {
                _ = other_paths.insert(pwd_path)
            }
        }
        Ok(Self { path, other_paths })
    }

    fn var_opt<K>(key: K) -> Result<Option<String>>
    where
        K: AsRef<OsStr>,
    {
        match var(key) {
            Ok(value) => Ok(Some(value)),
            Err(VarError::NotPresent) => Ok(None),
            _ => bail!("environment variable not found"),
        }
    }
}
