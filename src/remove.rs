use std::fs::remove_dir_all;

use crate::server::{Server, remove_server_from_list};

pub fn remove_server(name: &String) -> Result<(), String> {
    println!("[slapaman] removing server instance: {}", name);

    // first, check if the server exists
    let server = Server::load_by_name(name).unwrap();
    if server.name != *name {
        return Err(format!("server instance not found: {}", name));
    }

    // delete the server directory
    remove_dir_all(&server.path).unwrap();

    // remove the server from the list
    remove_server_from_list(&server).unwrap();

    println!("[slapaman] server instance removed: {}", name);
    Ok(())
}
