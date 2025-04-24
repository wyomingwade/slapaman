use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use std::path::PathBuf;

use crate::{init::ensure_slapaman_dir_exists, memory::parse_mem};

#[derive(Parser)]
#[command(name = "slapaman", version, about = "a command line tool for local Minecraft Java servers")]
pub struct Cli {
    /// increase log verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// create a new server instance
    New {
        name: String,
        #[arg(long, value_parser = parse_slapaman_dir)]
        path: Option<PathBuf>,
        #[arg(long, default_value = "release-latest")]
        version: String, // this will be converted to a Version struct
        #[arg(long, default_value = "false")]
        ignore_eula: bool,
    },
    /// remove an existing instance
    Remove {
        name: String,
    },
    /// start an existing instance
    Run { 
        name: String,
        #[arg(long, value_parser = parse_mem)]
        memory: Option<u32>, // MiB
        #[arg(long, default_value = "false")]
        quiet: bool,
    },
    /// list all instances
    List,
    Update {
        /// which instance to update (omit = all)
        name: Option<String>,
    },
    /// set the world for an instance to a pre-existing world
    WorldSet {
        name: String,
        world_path: PathBuf,
    },
}

fn parse_slapaman_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    Ok(path)
}