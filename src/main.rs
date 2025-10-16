// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

pub mod args;
pub mod init;
pub mod memory;
pub mod server;
pub mod version;

pub mod backup;
pub mod create;
pub mod remove;
pub mod run;
pub mod update;
pub mod world;

use clap::Parser;

use args::{Cli, Commands};
use backup::{create_world_backup, restore_world_backup};
use create::create_new_server;
use init::slapaman_init;
use remove::remove_server;
use run::run_server;
use server::{copy_server, list_servers, move_server, rename_server};
use update::{update_all_servers, update_server};
use version::Version;
use world::set_world;

#[tokio::main]
async fn main() {
    slapaman_init().unwrap();

    let cli = Cli::parse();

    // set up logging based on cli.verbose …
    match cli.command {
        Commands::New {
            name,
            path,
            version,
            ignore_eula,
        } => {
            match create_new_server(
                cli.verbose,
                path,
                name.clone(),
                Version::from_string(version),
                ignore_eula,
            )
            .await
            {
                Ok(_) => println!("[slapaman] created server instance: {}", name),
                Err(e) => println!("[slapaman] error creating server instance: {}", e),
            }
        }
        Commands::Rename { name, new_name } => match rename_server(&name, &new_name) {
            Ok(_) => println!(
                "[slapaman] renamed server instance: {} -> {}",
                name, new_name
            ),
            Err(e) => println!("[slapaman] error renaming server instance: {}", e),
        },
        Commands::Copy { name, new_name } => match copy_server(&name, &new_name) {
            Ok(_) => println!(
                "[slapaman] copied server instance: {} -> {}",
                name, new_name
            ),
            Err(e) => println!("[slapaman] error copying server instance: {}", e),
        },
        Commands::Move { name, new_path } => match move_server(&name, &new_path) {
            Ok(_) => println!(
                "[slapaman] moved server instance: {} -> {}",
                name,
                new_path.display()
            ),
            Err(e) => println!("[slapaman] error moving server instance: {}", e),
        },
        Commands::Remove { name } => match remove_server(&name) {
            Ok(_) => println!("[slapaman] removed server instance: {}", name),
            Err(e) => println!("[slapaman] error removing server instance: {}", e),
        },
        Commands::Run {
            name,
            memory,
            quiet,
        } => match run_server(cli.verbose, name.clone(), memory, Some(quiet)) {
            Ok(_) => println!("[slapaman] successfully ran server instance: {}", name),
            Err(e) => println!("[slapaman] error running server instance: {}", e),
        },
        Commands::List { detailed } => match list_servers(detailed) {
            Ok(_) => println!("[slapaman] successfully listed server instances"),
            Err(e) => println!("[slapaman] error listing server instances: {}", e),
        },
        Commands::Update { name, version } => {
            match update_server(&name, Version::from_string(version)).await {
                Ok(_) => println!("[slapaman] successfully updated server instance: {}", name),
                Err(e) => println!("[slapaman] error updating server instance: {}", e),
            }
        }
        Commands::UpdateAll { version } => {
            match update_all_servers(Version::from_string(version)).await {
                Ok(_) => println!("[slapaman] successfully updated all server instances"),
                Err(e) => println!("[slapaman] error updating all server instances: {}", e),
            }
        }
        Commands::WorldBackup { name, tag } => {
            match create_world_backup(cli.verbose, name.clone(), tag) {
                Ok(path) => println!(
                    "[slapaman] created world backup for server instance: {} -> {}",
                    name,
                    path.display()
                ),
                Err(e) => println!("[slapaman] error creating world backup: {}", e),
            }
        }
        Commands::WorldRestore { name, backup } => {
            match restore_world_backup(cli.verbose, name.clone(), &backup) {
                Ok(_) => println!(
                    "[slapaman] restored world backup for server instance: {}",
                    name
                ),
                Err(e) => println!("[slapaman] error restoring world backup: {}", e),
            }
        }
        Commands::WorldSet { name, world_path } => {
            match set_world(cli.verbose, name.clone(), &world_path) {
                Ok(_) => println!(
                    "[slapaman] successfully set world for server instance: {}",
                    name
                ),
                Err(e) => println!("[slapaman] error setting world for server instance: {}", e),
            }
        }
    }
}
