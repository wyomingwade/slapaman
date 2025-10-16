// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use directories::ProjectDirs;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Clone)]
pub struct Server {
    // stuff that must be known upon creation
    pub name: String,
    pub path: PathBuf,
    pub version: String,
    pub flavor: String,
    // can be configured after creation
    pub banned_ips: Value,
    pub banned_players: Value,
    pub eula: bool,
    pub whitelist: Value,
    pub ops: Value,
    pub permissions: Value,
    pub server_properties: Value,
}

impl Server {
    pub fn new(name: &String, path: &PathBuf, version: &String, flavor: &String) -> Self {
        Self {
            name: name.clone(),
            path: path.clone(),
            version: version.clone(),
            flavor: flavor.clone(),
            banned_ips: Value::Null,
            banned_players: Value::Null,
            eula: false,
            whitelist: Value::Null,
            ops: Value::Null,
            permissions: Value::Null,
            server_properties: Value::Null,
        }
    }

    // use this to load a server from slapaman's master list by name
    pub fn load_by_name(name: &String) -> Result<Self, String> {
        // load servers list
        let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
            .expect("could not determine a home directory")
            .data_dir()
            .join("servers.lock");

        let servers = load_servers_list(&servers_list).unwrap_or_default();

        // find the server in the list
        for server in servers {
            if server.name == *name {
                return Ok(server);
            }
        }

        // if the server is not found, return an error
        Err(format!("server not found: {}", name))
    }
}

fn load_servers_list(servers_list: &PathBuf) -> Result<Vec<Server>, String> {
    let file =
        File::open(servers_list).map_err(|e| format!("failed to open servers list: {}", e))?;
    let reader = BufReader::new(file);
    let servers: Vec<Server> = serde_json::from_reader(reader)
        .map_err(|e| format!("failed to parse servers list: {}", e))?;
    Ok(servers)
}

fn save_servers_list(servers_list: &PathBuf, servers: Vec<Server>) -> Result<(), String> {
    let file =
        File::create(servers_list).map_err(|e| format!("failed to create servers list: {}", e))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &servers)
        .map_err(|e| format!("failed to write servers list: {}", e))?;
    Ok(())
}

pub fn list_servers(detailed: bool) -> Result<(), String> {
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    let servers = load_servers_list(&servers_list).unwrap_or_default();

    match detailed {
        // when running quietly, just print the server names
        false => {
            for server in servers {
                println!("{}", server.name);
            }
        }
        // when running with the --detailed flag, print the server names, paths, and versions
        true => {
            for server in servers {
                println!(
                    "{}: {} ({})",
                    server.name,
                    server.path.display(),
                    server.version
                );
            }
        }
    }

    Ok(())
}

pub fn add_server_to_list(server: &Server) -> Result<(), String> {
    // path for where the servers list is stored
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    // load the servers list and add the new server
    let mut servers = load_servers_list(&servers_list).unwrap_or_default();
    servers.push(server.clone());

    // save the servers list
    save_servers_list(&servers_list, servers).unwrap();

    Ok(())
}

pub fn remove_server_from_list(server: &Server) -> Result<(), String> {
    // path for where the servers list is stored
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    // load the servers list and remove the server
    let mut servers = load_servers_list(&servers_list).unwrap_or_default();
    servers.remove(servers.iter().position(|s| s.name == server.name).unwrap());

    // save the servers list
    save_servers_list(&servers_list, servers).unwrap();

    Ok(())
}

pub fn rename_server(name: &String, new_name: &String) -> Result<(), String> {
    // this will load the server from slapaman's master list by name
    // fails when the server is not found
    let server_old = Server::load_by_name(&name).unwrap();

    // make sure the new name is not already taken
    if Server::load_by_name(&new_name).is_ok() {
        return Err(format!("server name already taken: {}", new_name));
    }

    // attempt to rename the server directory
    let old_path = server_old.path.join(&name).clone();
    let new_path = server_old.path.join(&new_name).clone();
    fs::rename(&old_path, &new_path)
        .map_err(|e| format!("failed to rename server directory: {}", e))?;

    // update the server's name in slapaman's master list
    let mut server_new = server_old.clone();
    server_new.name = new_name.to_string();
    remove_server_from_list(&server_old).unwrap();
    add_server_to_list(&server_new).unwrap();

    Ok(())
}

pub fn update_server_by_name(name: &String, server: &Server) -> Result<(), String> {
    // path for where the servers list is stored
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    // load the servers list
    let mut servers = load_servers_list(&servers_list).unwrap_or_default();

    // find the server in the list
    for server in servers.clone() {
        if server.name == *name {
            servers.remove(servers.iter().position(|s| s.name == server.name).unwrap());
        }
    }

    // add the updated server to the list
    servers.push(server.clone());

    // save the servers list
    save_servers_list(&servers_list, servers).unwrap();

    Ok(())
}

pub fn copy_server(name: &String, new_name: &String) -> Result<(), String> {
    // path for where the servers list is stored
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    // this will load the server from slapaman's master list by name
    // fails when the server is not found
    let server = Server::load_by_name(&name).unwrap();

    // make sure the new name is not already taken
    if Server::load_by_name(&new_name).is_ok() {
        return Err(format!("server name already taken: {}", new_name));
    }

    // copy the server directory
    let old_paths = server
        .path
        .join(&name)
        .clone()
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<PathBuf>>();
    let new_path = server.path.join(&new_name).clone();
    if !new_path.exists() {
        fs::create_dir_all(&new_path)
            .map_err(|e| format!("failed to create server directory: {}", e))?;
    }
    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: true,
        content_only: false,
        depth: 0,
    };
    copy_items(&old_paths, &new_path, &options)
        .map_err(|e| format!("failed to copy server directory: {}", e))?;

    // add copied server to slapaman's master list
    let mut servers = load_servers_list(&servers_list).unwrap_or_default();
    let mut server_new = server.clone();
    server_new.name = new_name.to_string();
    servers.push(server_new);
    save_servers_list(&servers_list, servers).unwrap();

    Ok(())
}

pub fn move_server(name: &String, new_path: &PathBuf) -> Result<(), String> {
    // path for where the servers list is stored
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    // this will load the server from slapaman's master list by name
    // fails when the server is not found
    let server = Server::load_by_name(&name).unwrap();

    // make sure the new path is not already taken
    let servers = load_servers_list(&servers_list).unwrap_or_default();
    for server in servers {
        if server.path == *new_path {
            return Err(format!("server path already taken: {}", new_path.display()));
        }
    }

    // move the server directory
    let old_path = server.path.join(&name).clone();
    let new_path = new_path.join(&name).clone();
    fs::rename(&old_path, &new_path)
        .map_err(|e| format!("failed to move server directory: {}", e))?;

    // update the server's path in slapaman's master list
    let mut server_new = server.clone();
    server_new.path = new_path.clone();
    remove_server_from_list(&server).unwrap();
    add_server_to_list(&server_new).unwrap();
    Ok(())
}

pub fn get_all_servers() -> Result<Vec<Server>, String> {
    let servers_list = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .expect("could not determine a home directory")
        .data_dir()
        .join("servers.lock");

    let servers = load_servers_list(&servers_list).unwrap_or_default();
    Ok(servers)
}

pub fn does_server_exist(name: &String) -> bool {
    let servers = get_all_servers().unwrap();
    for server in servers {
        if server.name == *name {
            return true;
        }
    }
    false
}
