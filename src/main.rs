mod args;
mod hex_digest;
mod init_command;
mod link;
mod link_command;
mod list_command;
mod manifest;
mod metadir;
mod remove_command;
mod repo;
mod show_command;

use crate::args::{Args, Subcommand};
use crate::init_command::do_init;
use crate::link_command::do_link;
use crate::list_command::do_list;
use crate::remove_command::do_remove;
use crate::repo::Repo;
use crate::show_command::do_show;
use anyhow::{anyhow, Result};
use clap::Parser;
use path_absolutize::Absolutize;
use std::env::{current_dir, set_var};
use std::path::{Path, PathBuf};

fn get_repo_dir(project_dir: &Path, args: &Args) -> Result<PathBuf> {
    Ok(match &args.repo_dir {
        Some(repo_dir) => repo_dir.absolutize_from(project_dir)?.to_path_buf(),
        _ => {
            let mut repo_dir = home::home_dir().ok_or(anyhow!("cannot get home directory"))?;
            repo_dir.push(".faf");
            repo_dir
        }
    })
}

fn main() -> Result<()> {
    set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();
    let project_dir = current_dir()?;
    let repo_dir = get_repo_dir(&project_dir, &args)?;
    let repo = Repo::new(&repo_dir);
    match args.subcommand {
        Some(Subcommand::Init) => do_init(&repo, &project_dir)?,
        Some(Subcommand::Link { meta_id }) => do_link(&repo, &meta_id, &project_dir)?,
        Some(Subcommand::List) => do_list(&repo)?,
        Some(Subcommand::Remove) => do_remove(&repo, &project_dir)?,
        Some(Subcommand::Show) => do_show(&repo, &project_dir)?,
        None => do_show(&repo, &project_dir)?,
    }
    Ok(())
}
