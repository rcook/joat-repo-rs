mod cli;

use crate::cli::{
    do_info, do_init, do_link, do_list, do_purge, do_remove, do_show, do_trash, Args, Logger,
    Status, Subcommand,
};
use anyhow::{anyhow, Result};
use clap::Parser;
use joat_repo::{Repo, RepoConfig};
use log::error;
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
            repo_dir.push(".joat-repo-test");
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

    if let Some(repo) = RepoConfig::default(&repo_dir, None).repo()? {
        run_command(&args, &repo, &project_dir)
    } else {
        error!(
            "Repository at {} is currently in use by another program or lock file is invalid",
            repo_dir.display()
        );
        Ok(Status::Failure)
    }
}

fn run_command(args: &Args, repo: &Repo, project_dir: &Path) -> Result<Status> {
    match &args.subcommand {
        Subcommand::Info => do_info(repo),
        Subcommand::Init => do_init(repo, project_dir),
        Subcommand::Link { meta_id } => do_link(repo, meta_id, project_dir),
        Subcommand::List => do_list(repo),
        Subcommand::Purge { force } => do_purge(repo, *force),
        Subcommand::Remove => do_remove(repo, project_dir),
        Subcommand::Show => do_show(repo, project_dir),
        Subcommand::Trash { clean } => do_trash(repo, *clean),
    }
}
