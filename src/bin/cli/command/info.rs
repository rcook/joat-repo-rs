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
use super::super::Status;
use anyhow::Result;
use colored::Colorize;
use joat_repo::Repo;
use std::path::Path;

pub fn do_info(repo: &Repo) -> Result<Status> {
    show_path("Lock file          ", &repo.config().lock_path);
    show_path("Configuration file ", &repo.config().config_path);
    show_path("Links directory    ", &repo.config().links_dir);
    show_path("Container directory", &repo.config().container_dir);
    show_path("Shared directory   ", &repo.config().shared_dir);
    Ok(Status::Success)
}

fn show_path(label: &str, path: &Path) {
    println!(
        "{}: {}",
        label.green(),
        format!("{}", path.display()).yellow()
    );
}
