use fs_extra::{copy_items, dir::CopyOptions};
use std::fs;
use std::path::PathBuf;

use crate::server::Server;

// set the world for a server instance to a pre-existing world
pub fn set_world(
    // slapaman params
    verbose: u8,
    // command args
    name: String,
    world_path: &PathBuf,
) -> Result<(), String> {
    println!("[slapaman] setting world for server instance: {}", name);

    let server = Server::load_by_name(&name).unwrap();

    // check if world_path is a valid world
    if !world_path.exists() {
        return Err(format!("world does not exist: {}", world_path.display()));
    }

    // check if world_path is a valid world (part 1)
    if !world_path.is_dir() {
        return Err(format!(
            "world is not a directory: {}",
            world_path.display()
        ));
    }

    // check if world_path is a valid world (part 2)
    if !world_path.join("level.dat").exists() {
        return Err(format!(
            "world is not a valid world: {}",
            world_path.display()
        ));
    }

    // get the server's world directory
    let server_dir = server.path.join(&name).join("world");

    // remove existing world directory if it exists
    if server_dir.exists() {
        fs::remove_dir_all(&server_dir)
            .map_err(|e| format!("failed to remove existing world: {}", e))?;
    }

    // create the world directory
    fs::create_dir_all(&server_dir)
        .map_err(|e| format!("failed to create world directory: {}", e))?;

    // copy the contents of the world to the server's world directory
    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: true,
        content_only: false,
        depth: 0,
    };

    let world_dir_contents = world_path
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<PathBuf>>();
    copy_items(&world_dir_contents, &server_dir, &options)
        .map_err(|e| format!("failed to copy world contents: {}", e))?;
    // ...that was way harder than it should have been

    println!("[slapaman] world set for server instance: {}", name);
    Ok(())
}
