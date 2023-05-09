mod args;
mod dir;
mod hash;
mod init_command;
mod manifest;
mod metadata;
mod remove_command;
mod repo;
mod show_command;

use crate::args::{Args, Subcommand};
use crate::dir::Dir;
use crate::init_command::do_init;
use crate::remove_command::do_remove;
use crate::repo::Repo;
use crate::show_command::do_show;
use anyhow::{anyhow, Result};
use clap::Parser;
use path_absolutize::Absolutize;
use std::path::PathBuf;

fn get_repo_dir(args: &Args) -> Result<PathBuf> {
    Ok(match &args.repo_dir {
        Some(repo_dir) => repo_dir.absolutize()?.to_path_buf(),
        _ => {
            let mut repo_dir = home::home_dir().ok_or(anyhow!("cannot get home directory"))?;
            repo_dir.push(".faf");
            repo_dir
        }
    })
}

fn main() -> Result<()> {
    let args = Args::parse();
    let repo_dir = get_repo_dir(&args)?;
    let repo = Repo::new(&repo_dir);
    let current_dir = Dir::from_cwd()?;
    match args.subcommand {
        Some(Subcommand::Init) => do_init(&repo, &current_dir)?,
        Some(Subcommand::Remove) => do_remove(&repo, &current_dir)?,
        Some(Subcommand::Show) => do_show(&repo, &current_dir)?,
        None => do_show(&repo, &current_dir)?,
    }
    Ok(())
}
