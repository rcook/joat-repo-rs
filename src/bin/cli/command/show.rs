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
use super::super::util::print;
use super::super::Status;
use anyhow::Result;
use joat_repo::Repo;
use log::error;
use std::path::Path;

pub fn do_show(repo: &Repo, project_dir: &Path) -> Result<Status> {
    Ok(match repo.get(project_dir)? {
        Some(dir_info) => {
            print("Data directory            ", dir_info.data_dir().display());
            print(
                "Manifest path             ",
                dir_info.manifest_path().display(),
            );
            print("Data directory created at ", dir_info.created_at());
            print(
                "Original project directory",
                dir_info.original_project_dir().display(),
            );
            print("Meta ID                   ", dir_info.meta_id());
            print("Link path                 ", dir_info.link_path().display());
            print("Link created at           ", dir_info.link_created_at());
            print("Link ID                   ", dir_info.link_id());
            print(
                "Project directory         ",
                dir_info.project_dir().display(),
            );
            Status::Success
        }
        None => {
            error!(
                "No metadirectory found for directory {}",
                project_dir.display()
            );
            Status::Failure
        }
    })
}
