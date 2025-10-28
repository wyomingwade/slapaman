// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use crate::server::{get_all_servers, update_server_by_name, Server};
use crate::version::{download_server_version, format_version_string, Version};

// basically, replace the server.jar file with a new one while preserving everything else
pub async fn update_server(
    name: &String,
    version: Version,
    flavor_override: Option<String>,
) -> Result<(), String> {
    // load the server
    // this will fail if the server doesn't exist
    let server = Server::load_by_name(&name).unwrap();

    let target_flavor = flavor_override
        .map(|f| f.to_lowercase())
        .unwrap_or_else(|| server.flavor.clone());

    // validate the given version
    let version_string = format_version_string(&version).await;
    if version_string == server.version && target_flavor == server.flavor {
        return Err(format!(
            "server is already on the given version and flavor: {} ({})",
            version_string, target_flavor
        ));
    }

    // download the new version
    download_server_version(&version, &target_flavor, &server.path, &server.name, true)
        .await
        .unwrap();

    // update the server's version in slapaman's master list
    let mut server_new = server.clone();
    server_new.version = version_string;
    server_new.flavor = target_flavor;
    update_server_by_name(&name, &server_new).unwrap();

    Ok(())
}

pub async fn update_all_servers(version: Version) -> Result<(), String> {
    let servers = get_all_servers().unwrap();
    for server in servers {
        // if an individual server update fails, note that, but continue updating the rest
        if update_server(&server.name, version.clone(), None)
            .await
            .is_err()
        {
            println!("[slapaman] error updating server instance: {}", server.name);
        }
    }

    Ok(())
}
