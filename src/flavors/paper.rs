// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::net::http::get_request;
use crate::version::Version;

pub async fn get_paper_version_bytes(game_version_id: &Version) -> Result<Vec<u8>, String> {
    let version_url = get_paper_version_url(game_version_id)?;
    let bytes = download_paper_jar(&version_url).await?;
    Ok(bytes)
}

fn get_paper_version_url(game_version_id: &Version) -> Result<String, String> {
    let url = format!(
        "https://fill.papermc.io/v3/projects/paper/versions/{}/builds",
        &game_version_id.v_id
    );
    Ok(url)
}

async fn download_paper_jar(version_url: &String) -> Result<Vec<u8>, String> {
    // make HTTP request
    let response = match get_request(version_url).await {
        Ok(response) => response,
        Err(e) => return Err(format!("failed to send GET request: {}", e)),
    };
    let body = response.text().await.unwrap();

    // parse JSON response
    let json: Value = serde_json::from_str(&body).unwrap();
    let latest_build = json.as_array().unwrap().first().unwrap();
    let jar_url = latest_build["downloads"]["server:default"]["url"]
        .as_str()
        .unwrap()
        .to_string();
    let jar_checksum = latest_build["downloads"]["server:default"]["checksums"]["sha256"]
        .as_str()
        .unwrap()
        .to_string();
    let jar_size = latest_build["downloads"]["server:default"]["size"]
        .as_u64()
        .unwrap();

    // download the jar
    let jar_response = match get_request(&jar_url.to_string()).await {
        Ok(response) => response,
        Err(e) => return Err(format!("failed to send GET request: {}", e)),
    };
    let jar_body = jar_response.bytes().await.unwrap();

    // verify the size and checksum
    if jar_body.len() != jar_size as usize {
        return Err(format!(
            "size mismatch: got {}, expected {}",
            jar_body.len(),
            jar_size
        ));
    }
    let mut hasher = Sha256::new();
    hasher.update(&jar_body);
    let computed_hash = format!("{:x}", hasher.finalize());
    if computed_hash != jar_checksum {
        return Err(format!(
            "checksum mismatch: got {}, expected {}",
            computed_hash, jar_checksum
        ));
    }

    Ok(jar_body.to_vec())
}
