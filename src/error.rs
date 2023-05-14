// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use anyhow::Error as AnyhowError;
use joatmon::HasOtherError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::SharedPath;

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum RepoErrorKind {
    CouldNotOpenLockFile,
    CouldNotLock,
    InvalidSharedPath,
    CouldNotComputeHash,
    CouldNotDeleteDirectory,
    CouldNotDeleteFile,
    InvalidLinkFile,
    Other,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct RepoError(#[from] RepoErrorImpl);

#[derive(Debug, Error)]
enum RepoErrorImpl {
    #[error("Could not open lock file {0}")]
    CouldNotOpenLockFile(PathBuf),
    #[error("Could not lock lock file {0}")]
    CouldNotLock(PathBuf),
    #[error("Invalid shared path {0}")]
    InvalidSharedPath(SharedPath),
    #[error("Could not compute MD5 hash for path {0}")]
    CouldNotComputeHash(PathBuf),
    #[error("Could not delete directory {0}")]
    CouldNotDeleteDirectory(PathBuf),
    #[error("Could not delete file {0}")]
    CouldNotDeleteFile(PathBuf),
    #[error(
        "Project directory {1} specified in link file {0} does not match expected directory {2}"
    )]
    InvalidLinkFile(PathBuf, PathBuf, PathBuf),
    #[error(transparent)]
    Other(AnyhowError),
}

impl RepoError {
    #[allow(unused)]
    pub fn kind(&self) -> RepoErrorKind {
        match self.0 {
            RepoErrorImpl::CouldNotOpenLockFile(_) => RepoErrorKind::CouldNotOpenLockFile,
            RepoErrorImpl::CouldNotLock(_) => RepoErrorKind::CouldNotLock,
            RepoErrorImpl::InvalidSharedPath(_) => RepoErrorKind::InvalidSharedPath,
            RepoErrorImpl::CouldNotComputeHash(_) => RepoErrorKind::CouldNotComputeHash,
            RepoErrorImpl::CouldNotDeleteDirectory(_) => RepoErrorKind::CouldNotDeleteDirectory,
            RepoErrorImpl::CouldNotDeleteFile(_) => RepoErrorKind::CouldNotDeleteFile,
            RepoErrorImpl::InvalidLinkFile(_, _, _) => RepoErrorKind::InvalidLinkFile,
            _ => RepoErrorKind::Other,
        }
    }

    #[allow(unused)]
    pub fn is_could_not_open_lock_file(&self) -> bool {
        self.kind() == RepoErrorKind::CouldNotOpenLockFile
    }

    #[allow(unused)]
    pub fn is_could_not_lock(&self) -> bool {
        self.kind() == RepoErrorKind::CouldNotLock
    }

    #[allow(unused)]
    pub fn is_invalid_shared_path(&self) -> bool {
        self.kind() == RepoErrorKind::InvalidSharedPath
    }

    #[allow(unused)]
    pub fn is_could_not_compute_hash(&self) -> bool {
        self.kind() == RepoErrorKind::CouldNotComputeHash
    }

    #[allow(unused)]
    pub fn is_could_not_delete_directory(&self) -> bool {
        self.kind() == RepoErrorKind::CouldNotDeleteDirectory
    }

    #[allow(unused)]
    pub fn is_could_not_delete_file(&self) -> bool {
        self.kind() == RepoErrorKind::CouldNotDeleteFile
    }

    #[allow(unused)]
    pub fn is_invalid_link_file(&self) -> bool {
        self.kind() == RepoErrorKind::InvalidLinkFile
    }

    #[allow(unused)]
    pub fn is_other(&self) -> bool {
        self.kind() == RepoErrorKind::Other
    }

    pub(crate) fn could_not_open_lock_file(path: &Path) -> Self {
        Self(RepoErrorImpl::CouldNotOpenLockFile(path.to_path_buf()))
    }

    pub(crate) fn could_not_lock(path: &Path) -> Self {
        Self(RepoErrorImpl::CouldNotLock(path.to_path_buf()))
    }

    pub(crate) fn invalid_shared_path(shared_path: &SharedPath) -> Self {
        Self(RepoErrorImpl::InvalidSharedPath(shared_path.clone()))
    }

    pub(crate) fn could_not_compute_hash(project_dir: &Path) -> Self {
        Self(RepoErrorImpl::CouldNotComputeHash(
            project_dir.to_path_buf(),
        ))
    }

    pub(crate) fn could_not_delete_directory(path: &Path) -> Self {
        Self(RepoErrorImpl::CouldNotDeleteDirectory(path.to_path_buf()))
    }

    pub(crate) fn could_not_delete_file(path: &Path) -> Self {
        Self(RepoErrorImpl::CouldNotDeleteFile(path.to_path_buf()))
    }

    pub(crate) fn invalid_link_file(
        link_path: &Path,
        project_dir: &Path,
        expected_project_dir: &Path,
    ) -> Self {
        Self(RepoErrorImpl::InvalidLinkFile(
            link_path.to_path_buf(),
            project_dir.to_path_buf(),
            expected_project_dir.to_path_buf(),
        ))
    }

    pub(crate) fn other<E>(e: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(RepoErrorImpl::Other(AnyhowError::new(e)))
    }
}

impl HasOtherError for RepoError {
    fn is_other(&self) -> bool {
        self.is_other()
    }

    fn downcast_other_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let RepoErrorImpl::Other(inner) = &self.0 {
            inner.downcast_ref::<E>()
        } else {
            None
        }
    }
}
