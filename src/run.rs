use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::memory::memory_value_coerced;
use crate::server::Server;

pub fn run_server(
    // slapaman params 
    _verbose: u8,
    // command args
    name: String, 
    memory: Option<u32>,
    quiet: Option<bool>,
) -> Result<(), String> {
    println!("[slapaman] starting server: {}", &name);

    let server = Server::load_by_name(&name).unwrap();

    let server_dir = server.path.join(&name);

    // make sure the server directory exists
    if !server_dir.exists() {
        return Err(format!("server instance does not exist: {}", &name));
    }

    // make sure the server directory is a directory
    if !server_dir.is_dir() {
        return Err(format!("server instance is not a directory: {}", &name));
    }   

    // make sure the server directory is a valid server instance
    if !server_dir.join("server.jar").exists() {
        return Err(format!("server instance is not a valid server: {}", &name));
    }
    
    let server_jar = server_dir.join("server.jar");
    let memory = memory_value_coerced(memory);
    let run_quietly = runtime_quiet_coerced(quiet);

    // run the server
    let mut child = match run_quietly {
        // when running quietly, we don't want any output
        true => Command::new("java")
            .arg(format!("-Xmx{}M", memory))
            .arg(format!("-Xms{}M", memory))
            .arg("-jar")
            .arg(server_jar)
            .arg("-nogui")
            .current_dir(server_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to run server"),
        // when running verbosely, we want to see the output
        false => Command::new("java")
            .arg(format!("-Xmx{}M", memory))
            .arg(format!("-Xms{}M", memory))
            .arg("-jar")
            .arg(server_jar)
            .arg("-nogui")
            .current_dir(server_dir)
            .spawn()
            .expect("failed to run server"),
    };

    // Wait for the server to finish
    let status = child.wait().expect("failed to wait for server");

    if !status.success() {
        return Err(format!("server exited with error code: {}", status));
    }

    println!("[slapaman] server finished running: {}", name);

    Ok(())
}

fn runtime_quiet_coerced(quiet: Option<bool>) -> bool {
    match quiet {
        Some(true) => true,
        _ => false,
    }
}
