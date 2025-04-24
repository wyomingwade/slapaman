use std::path::PathBuf;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use directories::ProjectDirs;

use crate::version::Version;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Clone)]
pub struct Server {
    // stuff that must be known upon creation
    pub name: String,
    pub path: PathBuf,
    pub version: String,
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
    pub fn new(name: &String, path: &PathBuf, version: &String) -> Self {
        Self { 
            name: name.clone(), 
            path: path.clone(), 
            version: version.clone(), 
            banned_ips: Value::Null, 
            banned_players: Value::Null, 
            eula: false, 
            whitelist: Value::Null, 
            ops: Value::Null, 
            permissions: Value::Null, 
            server_properties: Value::Null 
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
    let file = File::open(servers_list).map_err(|e| format!("failed to open servers list: {}", e))?;
    let reader = BufReader::new(file);
    let servers: Vec<Server> = serde_json::from_reader(reader).map_err(|e| format!("failed to parse servers list: {}", e))?;
    Ok(servers)
}

fn save_servers_list(servers_list: &PathBuf, servers: Vec<Server>) -> Result<(), String> {
    let file = File::create(servers_list).map_err(|e| format!("failed to create servers list: {}", e))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &servers).map_err(|e| format!("failed to write servers list: {}", e))?;
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
