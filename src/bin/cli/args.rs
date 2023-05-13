use anyhow::Result;
use clap::Parser;
use clap::Subcommand as ClapSubcommand;
use faf::MetaId;
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
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    #[command(name = "info", about = "Show configuration")]
    Info,

    #[command(name = "init", about = "Initialize metadirectory")]
    Init,

    #[command(name = "ln", about = "Create link to existing metadirectory")]
    Link {
        #[arg(
            long = "ref",
            short = 'r',
            help = "Existing metadirectory ID",
            value_parser = parse_meta_id
        )]
        meta_id: Option<MetaId>,
    },

    #[command(name = "ls", about = "Show all metadirectory info")]
    List,

    #[command(name = "purge", about = "Purge repository")]
    Purge {
        #[arg(
            long = "force",
            default_value = "false",
            help = "Really purge repository"
        )]
        force: bool,
    },

    #[command(name = "rm", about = "Unlink metadirectory")]
    Remove,

    #[command(name = "show", about = "Show metadirectory info")]
    Show,

    #[command(name = "trash", about = "Show trash and optionally clean up")]
    Trash {
        #[arg(long = "clean", default_value = "false", help = "Clean up")]
        clean: bool,
    },
}

fn parse_meta_id(s: &str) -> Result<MetaId> {
    MetaId::parse(s)
}
