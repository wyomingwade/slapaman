pub mod init;
pub mod args;
pub mod memory;
pub mod version;
pub mod server;

pub mod create;
pub mod remove;
pub mod run;
// pub mod list;
// pub mod update;
pub mod world;

use clap::Parser;

use init::slapaman_init;
use args::{Cli, Commands};
use create::create_new_server;
use remove::remove_server;
use run::run_server;
use version::Version;
use world::set_world;

#[tokio::main]
async fn main() {
    slapaman_init().unwrap();

    let cli = Cli::parse();

    // set up logging based on cli.verbose …
    match cli.command {
        Commands::New { name, path, version, ignore_eula } => { 
            match create_new_server(cli.verbose, path, name.clone(), Version::from_string(version), ignore_eula).await {
                Ok(_) => println!("[slapaman] created server instance: {}", name),
                Err(e) => println!("[slapaman] error creating server instance: {}", e),
            }
        }
        Commands::Remove { name } => {
            match remove_server(&name) {
                Ok(_) => println!("[slapaman] removed server instance: {}", name),
                Err(e) => println!("[slapaman] error removing server instance: {}", e),
            }
        }
        Commands::Run { name, memory, quiet } => { 
            match run_server(cli.verbose, name.clone(), memory, Some(quiet)) {
                Ok(_) => println!("[slapaman] successfully ran server instance: {}", name),
                Err(e) => println!("[slapaman] error running server instance: {}", e),
            }
        }
        Commands::List                => { /* … */ }
        Commands::Update { name }     => { /* … */ }
        Commands::WorldSet { name, world_path } => {
            match set_world(cli.verbose, name.clone(), &world_path) {
                Ok(_) => println!("[slapaman] successfully set world for server instance: {}", name),
                Err(e) => println!("[slapaman] error setting world for server instance: {}", e),
            }
        }
    }
}