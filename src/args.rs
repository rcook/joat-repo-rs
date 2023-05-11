use crate::hex_digest::HexDigest;
use anyhow::Result;
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
    #[command(
        name = "alias",
        about = "Register directory as alias of existing directory with metadata"
    )]
    Alias {
        #[arg(name = "ref", value_parser = parse_hex_digest)]
        reference: HexDigest,
    },

    #[command(name = "init", about = "Initialize metadirectory")]
    Init,

    #[command(name = "ls", about = "Show all metadirectory info")]
    List,

    #[command(name = "rm", about = "Remove metadirectory")]
    Remove,

    #[command(name = "show", about = "Show metadirectory info")]
    Show,
}

fn parse_hex_digest(s: &str) -> Result<HexDigest> {
    Ok(HexDigest::new(s))
}
