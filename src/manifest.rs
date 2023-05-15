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
use crate::meta_id::MetaId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ManifestRecord {
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) original_project_dir: PathBuf,
    pub(crate) meta_id: MetaId,
}

#[derive(Clone, Debug)]
pub struct Manifest {
    data_dir: PathBuf,
    manifest_path: PathBuf,
    record: ManifestRecord,
}

impl Manifest {
    pub(crate) fn new(data_dir: PathBuf, manifest_path: PathBuf, record: ManifestRecord) -> Self {
        Self {
            data_dir,
            manifest_path,
            record,
        }
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.record.created_at
    }

    pub fn original_project_dir(&self) -> &Path {
        &self.record.original_project_dir
    }

    pub fn meta_id(&self) -> &MetaId {
        &self.record.meta_id
    }
}
