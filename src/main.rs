mod args;
mod clean_command;
mod hex_digest;
mod init_command;
mod link;
mod link_command;
mod list_command;
mod logger;
mod manifest;
mod metadir;
mod remove_command;
mod repo;
mod show_command;
mod status;
mod util;

use crate::args::{Args, Subcommand};
use crate::clean_command::do_clean;
use crate::init_command::do_init;
use crate::link_command::do_link;
use crate::list_command::do_list;
use crate::logger::Logger;
use crate::remove_command::do_remove;
use crate::repo::Repo;
use crate::show_command::do_show;
use crate::status::Status;
use anyhow::{anyhow, Result};
use clap::Parser;
use log::{set_logger, set_max_level, LevelFilter};
use path_absolutize::Absolutize;
use std::env::{current_dir, set_var, var, VarError};
use std::path::{Path, PathBuf};
use std::process::exit;

static LOGGER: Logger = Logger;

#[cfg(debug_assertions)]
fn init_backtrace() {
    const RUST_BACKTRACE_ENV_NAME: &str = "RUST_BACKTRACE";

    if let Err(VarError::NotPresent) = var(RUST_BACKTRACE_ENV_NAME) {
        set_var(RUST_BACKTRACE_ENV_NAME, "1")
    }

    color_backtrace::install();
}

#[cfg(not(debug_assertions))]
fn init_backtrace() {}

fn init_logger() -> Result<()> {
    set_logger(&LOGGER).map_err(|e| anyhow!(e))?;
    set_max_level(LevelFilter::Info);
    Ok(())
}

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
    init_backtrace();
    init_logger()?;

    exit(match run()? {
        Status::Success => 0,
        Status::Failure => 1,
    });
}

fn run() -> Result<Status> {
    let args = Args::parse();
    let project_dir = current_dir()?;
    let repo_dir = get_repo_dir(&project_dir, &args)?;
    let repo = Repo::new(&repo_dir);
    match args.subcommand {
        Subcommand::Clean { force } => do_clean(&repo, force),
        Subcommand::Init => do_init(&repo, &project_dir),
        Subcommand::Link { meta_id } => do_link(&repo, &meta_id, &project_dir),
        Subcommand::List => do_list(&repo),
        Subcommand::Remove => do_remove(&repo, &project_dir),
        Subcommand::Show => do_show(&repo, &project_dir),
    }
}
