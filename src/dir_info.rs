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
use crate::link::LinkEx;
use crate::link_id::LinkId;
use crate::manifest::ManifestEx;
use crate::meta_id::MetaId;
use chrono::{DateTime, Utc};
use std::path::Path;

#[derive(Debug)]
pub struct DirInfo {
    pub(crate) manifest: ManifestEx,
    pub(crate) link: LinkEx,
}

impl DirInfo {
    pub fn data_dir(&self) -> &Path {
        &self.manifest.data_dir
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest.manifest_path
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.manifest.manifest.created_at
    }

    pub fn original_project_dir(&self) -> &Path {
        &self.manifest.manifest.original_project_dir
    }

    pub fn meta_id(&self) -> &MetaId {
        &self.manifest.manifest.meta_id
    }

    pub fn link_path(&self) -> &Path {
        &self.link.link_path
    }

    pub fn link_created_at(&self) -> &DateTime<Utc> {
        &self.link.link.created_at
    }

    pub fn link_id(&self) -> &LinkId {
        &self.link.link.link_id
    }

    pub fn project_dir(&self) -> &Path {
        &self.link.link.project_dir
    }
}
