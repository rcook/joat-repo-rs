use clap::Parser;
use clap::Subcommand as ClapSubcommand;
use std::path::PathBuf;
use uuid::Uuid;

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
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    #[command(name = "clean", about = "Clean unreferenced metadirectories")]
    Clean,

    #[command(name = "init", about = "Initialize metadirectory")]
    Init,

    #[command(name = "ln", about = "Create link to existing metadirectory")]
    Link {
        #[arg(long = "ref", short = 'r', help = "Existing metadirectory ID")]
        meta_id: Option<Uuid>,
    },

    #[command(name = "ls", about = "Show all metadirectory info")]
    List,

    #[command(name = "rm", about = "Unlink metadirectory")]
    Remove,

    #[command(name = "show", about = "Show metadirectory info")]
    Show,
}
