// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use crate::net::http::get_request;
use crate::version::Version;

const LOADER_VERSION_DEFAULT: &str = "0.17.3";
const INSTALLER_VERSION_DEFAULT: &str = "1.1.0";

pub async fn get_fabric_version_bytes(
    game_version_id: &Version,
    fabric_loader_version: Option<&String>,
    fabric_installer_version: Option<&String>,
) -> Result<Vec<u8>, String> {
    let version_url = get_fabric_version_url(game_version_id, fabric_loader_version, fabric_installer_version)?;
    let bytes = download_fabric_jar(&version_url).await?;
    Ok(bytes)
}

fn get_fabric_version_url(
    game_version_id: &Version,
    fabric_loader_version: Option<&String>,
    fabric_installer_version: Option<&String>,
) -> Result<String, String> {
    let loader_version = match fabric_loader_version {
        Some(version) => version,
        None => &get_default_loader_from_game_version().unwrap(),
    };
    let installer_version = match fabric_installer_version {
        Some(version) => version,
        None => &get_default_installer_from_game_version().unwrap(),
    };

    let url = format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",
        &game_version_id.v_id,
        loader_version,
        installer_version
    );
    Ok(url)
}

async fn download_fabric_jar(version_url: &String) -> Result<Vec<u8>, String> {
    let response = match get_request(version_url).await {
        Ok(response) => response,
        Err(e) => return Err(format!("failed to send GET request: {}", e)),
    };
    let body = match response.bytes().await {
        Ok(body) => body,
        Err(e) => return Err(format!("failed to read response body: {}", e))
    };

    Ok(body.to_vec())
}

fn get_default_loader_from_game_version() -> Result<String, String> {
    Ok(LOADER_VERSION_DEFAULT.to_string())
}

fn get_default_installer_from_game_version() -> Result<String, String> {
    Ok(INSTALLER_VERSION_DEFAULT.to_string())
}