use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use directories::ProjectDirs;

use crate::version::{Version, download_server_version, format_version_string};
use crate::run::run_server;
use crate::server::{Server, add_server_to_list, update_server_by_name};

pub async fn create_new_server(
    // slapaman params 
    verbose: u8, 
    // command args
    path: Option<PathBuf>,
    name: String, 
    version: Version, 
    ignore_eula: bool,
) -> Result<(), String> {
    println!("[slapaman] creating new server instance: {}", name);

    let directory = server_dir_coerced(path);

    // create the servers directory if it doesn't exist
    if !directory.exists() {
        create_dir_all(&directory).unwrap();
    }

    let server_dir = directory.join(&name);

    // make sure the server directory doesn't already exist
    if server_dir.exists() {
        return Err(format!("server instance already exists: {}", &name));
    }

    // create the server directory
    create_dir_all(&server_dir).unwrap();

    // download the server version
    download_server_version(&version, &directory, &name, false).await.unwrap();

    let version_string = format_version_string(&version).await;
    // register the server in the servers.lock file
    let mut server = Server::new(&name, &directory, &version_string);
    add_server_to_list(&server).unwrap();

    // register the server in the servers.lock file
    let mut server = Server::new(&name, &directory, &version.to_string());
    add_server_to_list(&server).unwrap();

    // run the server for the first time
    // this will create the eula.txt file and various other files
    run_server(verbose, name.clone(), None, Some(true)).unwrap();

    // agree to the eula if the user didn't specify to ignore it
    if !ignore_eula {
        agree_to_eula(&server_dir).unwrap();
        server.eula = true;
        update_server_by_name(&name, &server).unwrap();
        println!("[slapaman] eula agreed to");
    }

    println!("[slapaman] created server instance: {}", &name);
    
    Ok(())
}

fn agree_to_eula(server_dir: &PathBuf) -> Result<(), String> {
    let eula_path = server_dir.join("eula.txt");
    let mut eula_file = File::create(eula_path).unwrap();
    eula_file.write_all(b"eula=true\n").unwrap();

    Ok(())
}

fn server_dir_coerced(path: Option<PathBuf>) -> PathBuf {
    match path {
        Some(p) => p,
        None => ProjectDirs::from("com", "wyomingwade", "slapaman").unwrap().data_dir().to_path_buf().join("servers"),
    }
}
