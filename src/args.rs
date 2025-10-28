// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::memory::parse_mem;

#[derive(Parser)]
#[command(
    name = "slapaman",
    author = "Wyoming Wade (GitHub: @wyomingwade)",
    version,
    about = "a command line tool for downloading, managing, and running local Minecraft Java servers"
)]
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
        /// the name of the server instance
        name: String,
        /// the path to the server instance (slapaman uses its own "servers" directory if none is given)
        #[arg(long, short, value_parser = parse_slapaman_dir)]
        path: Option<PathBuf>,
        /// the version of the server instance
        #[arg(long, short, default_value = "release-latest")]
        version: String, // this will be converted to a Version struct
        /// the flavor of the server instance (vanilla, forge, fabric, etc.)
        #[arg(long, short, default_value = "vanilla")]
        flavor: String,
        /// don't automatically accept the EULA upon first launch
        #[arg(long, short, default_value = "false")]
        ignore_eula: bool,
    },
    /// rename an existing instance
    Rename {
        /// the current name of the server instance
        name: String,
        /// the new name of the server instance
        new_name: String,
    },
    /// copy an existing instance
    Copy {
        /// the name of the server instance to copy
        name: String,
        /// the new name of the server instance
        new_name: String,
    },
    /// move an existing instance to a new directory
    Move {
        /// the name of the server instance to move
        name: String,
        /// the new path of the server instance
        new_path: PathBuf,
    },
    /// remove an existing instance
    Remove {
        /// the name of the server instance
        name: String,
    },
    /// start an existing instance
    Run {
        /// the name of the server instance
        name: String,
        /// the amount of memory to allocate to the server instance (2048M if none is given)
        #[arg(long, short,value_parser = parse_mem)]
        memory: Option<u32>, // MiB
        /// suppress output from the server instance (generally not recommended)
        #[arg(long, short, default_value = "false")]
        quiet: bool,
    },
    /// list all instances
    List {
        /// whether to list all instances in detail
        #[arg(long, short, default_value = "false")]
        detailed: bool,
    },
    /// update an existing instance to a new version
    Update {
        /// which instance to update (omit = all)
        name: String,
        /// the version of the server instance
        #[arg(long, short, default_value = "release-latest")]
        version: String, // this will be converted to a Version struct
        /// the flavor of the server instance (omit to keep current)
        #[arg(long, short)]
        flavor: Option<String>,
    },
    /// update all instances to a new version
    UpdateAll {
        /// the version of the server instance
        #[arg(long, short, default_value = "release-latest")]
        version: String, // this will be converted to a Version struct
        /// the flavor of the server instance (omit to keep current)
        #[arg(long, short)]
        flavor: Option<String>,
    },
    /// create a backup of an instance's world
    WorldBackup {
        /// the name of the server instance
        name: String,
        /// optional tag appended to the backup directory name
        #[arg(long, short)]
        tag: Option<String>,
    },
    /// restore an instance's world from a backup
    WorldRestore {
        /// the name of the server instance
        name: String,
        /// the path or identifier of the backup directory to restore
        backup: PathBuf,
    },
    /// set the world for an instance to a pre-existing world
    WorldSet {
        /// the name of the server instance
        name: String,
        /// the path to the world file to copy over to the server instance
        world_path: PathBuf,
    },
    /// create a backup of an instance's overworld dimension
    OverworldBackup {
        /// the name of the server instance
        name: String,
        /// optional tag appended to the backup directory name
        #[arg(long, short)]
        tag: Option<String>,
    },
    /// restore an instance's overworld dimension from a backup
    OverworldRestore {
        /// the name of the server instance
        name: String,
        /// the path or identifier of the backup directory to restore
        backup: PathBuf,
    },
    /// set the overworld dimension for an instance to a pre-existing overworld
    OverworldSet {
        /// the name of the server instance
        name: String,
        /// the path to the overworld directory to copy over to the server instance
        overworld_path: PathBuf,
    },
    /// create a backup of an instance's nether dimension
    NetherBackup {
        /// the name of the server instance
        name: String,
        /// optional tag appended to the backup directory name
        #[arg(long, short)]
        tag: Option<String>,
    },
    /// restore an instance's nether dimension from a backup
    NetherRestore {
        /// the name of the server instance
        name: String,
        /// the path or identifier of the backup directory to restore
        backup: PathBuf,
    },
    /// set the nether dimension for an instance to a pre-existing nether
    NetherSet {
        /// the name of the server instance
        name: String,
        /// the path to the nether directory to copy over to the server instance
        nether_path: PathBuf,
    },
    /// create a backup of an instance's end dimension
    EndBackup {
        /// the name of the server instance
        name: String,
        /// optional tag appended to the backup directory name
        #[arg(long, short)]
        tag: Option<String>,
    },
    /// restore an instance's end dimension from a backup
    EndRestore {
        /// the name of the server instance
        name: String,
        /// the path or identifier of the backup directory to restore
        backup: PathBuf,
    },
    /// set the end dimension for an instance to a pre-existing end
    EndSet {
        /// the name of the server instance
        name: String,
        /// the path to the end directory to copy over to the server instance
        end_path: PathBuf,
    },
}

fn parse_slapaman_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    Ok(path)
}
