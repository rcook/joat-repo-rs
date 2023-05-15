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
mod config;
mod dir_info;
mod error;
mod link;
mod link_id;
mod manifest;
mod meta_id;
mod repo;
mod result;
mod shared_path;
mod trash;

pub use self::config::RepoConfig;
pub use self::dir_info::DirInfo;
pub use self::error::{RepoError, RepoErrorKind};
pub use self::link::Link;
pub use self::link_id::LinkId;
pub use self::manifest::Manifest;
pub use self::meta_id::MetaId;
pub use self::repo::Repo;
pub use self::result::RepoResult;
pub use self::shared_path::SharedPath;
pub use self::trash::Trash;
