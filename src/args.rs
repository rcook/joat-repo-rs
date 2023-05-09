use clap::Parser;
use clap::Subcommand as ClapSubcommand;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        global = true,
        long = "dir",
        short = 'd',
        help = "Path to repository directory"
    )]
    pub repo_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    #[command(name = "init", about = "Initialize metadata for directory")]
    Init,

    #[command(name = "rm", about = "Remove metadata for directory")]
    Remove,

    #[command(name = "show", about = "Show metadata for directory")]
    Show,
}
