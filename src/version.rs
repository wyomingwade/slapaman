use clap::ValueEnum;
use serde_json::Value;
use std::{fs::File, path::PathBuf};
use std::io::Write;
use sha1::{Sha1, Digest};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Version {
    pub v_id: String, // the version ID or "latest"
    pub v_type: VersionType,
}

impl Version {
    pub fn new(v_id: String, v_type: VersionType) -> Self {
        Self { v_id, v_type }
    }

    // create a Version struct from the string the user passes in
    // examples: "release-latest", "snapshot-latest", "release-1.20.1", "snapshot-25w17a"
    pub fn from_string(version_string: String) -> Self {
        let v_type = if version_string.contains("snapshot") {
            VersionType::Snapshot
        } else {
            VersionType::Release
        };

        let v_id = version_string.split("-").last().unwrap();

        Self::new(v_id.to_string(), v_type)
    }

    pub fn to_string(&self) -> String {
        match self.v_type {
            VersionType::Snapshot => format!("snapshot-{}", self.v_id),
            VersionType::Release => format!("release-{}", self.v_id),
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq, Copy)]
pub enum VersionType {
    Snapshot,
    Release,
}

// download the servere.jar file for specified version from Mojang's API
pub async fn download_server_version(version_id: &Version, default_directory: &PathBuf, server_name: &str, overwrite_existing: bool) -> Result<(), String> {
    // fetch the manifest of versions from Mojang's API
    let manifest = fetch_manifest().await.unwrap();

    // get the relevant version URL from the manifest
    let version_url = resolve_version(&manifest, version_id).unwrap();

    // download the server.jar file for this version
    let server_jar_bytes = download_version_from_url(&version_url).await.unwrap();

    // save the server.jar file to {default_directory}/{server_name}/server.jar
    let server_jar_path = default_directory.join(server_name).join("server.jar");
    if !overwrite_existing && server_jar_path.exists() {
        return Err(format!("[slapaman] server.jar file already exists: {}", server_jar_path.display()));
    }
    let mut file = File::create(server_jar_path).unwrap();
    file.write_all(&server_jar_bytes).unwrap();

    Ok(())
}

// get the manifest of versions from Mojang's API
async fn fetch_manifest() -> Result<Value, String> {
    // send GET request to Mojang's API
    let url = "https://piston-meta.mojang.com/mc/game/version_manifest.json";
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();

    // parse the manifest
    let manifest: Value = serde_json::from_str(&body).unwrap();

    Ok(manifest)
}

// get the relevant version URL from the manifest
fn resolve_version(manifest: &Value, version_id: &Version) -> Result<String, String> {
    // just look at how rusty this whole thing is
    let version: Value = match version_id.v_id.as_str() {
        "latest" => {
            if version_id.v_type == VersionType::Snapshot {
                let latest_snapshot = manifest["latest"]["snapshot"].as_str().unwrap();
                manifest["versions"].as_array().unwrap().iter().find(|v| v["id"].as_str() == Some(latest_snapshot)).unwrap().clone()
            } else {
                let latest_release = manifest["latest"]["release"].as_str().unwrap();
                manifest["versions"].as_array().unwrap().iter().find(|v| v["id"].as_str() == Some(latest_release)).unwrap().clone()
            }
        }
        _ => {
            manifest["versions"].as_array().unwrap().iter().find(|v| v["id"].as_str() == Some(version_id.v_id.as_str())).unwrap().clone()
        }
    };

    Ok(version["url"].as_str().unwrap().to_string())
}

// download the version from the URL and verify the size and SHA1 hash
async fn download_version_from_url(version_url: &str) -> Result<Vec<u8>, String> {
    // send GET request to the version URL
    let response = reqwest::get(version_url).await.unwrap();
    let body = response.text().await.unwrap();

    // parse the version
    let version_json: Value = serde_json::from_str(&body).unwrap();

    let server_jar_url = version_json["downloads"]["server"]["url"].as_str().unwrap();
    let version_size = version_json["downloads"]["server"]["size"].as_u64().unwrap();
    let version_sha1 = version_json["downloads"]["server"]["sha1"].as_str().unwrap();

    // download the version
    let server_jar_bytes = reqwest::get(server_jar_url).await.unwrap().bytes().await.unwrap();

    // verify the size
    if server_jar_bytes.len() != version_size as usize {
        return Err(format!("[slapaman] size mismatch: {} != {}", server_jar_bytes.len(), version_size));
    }
    
    // verify the SHA1 hash
    let mut hasher = Sha1::new();
    hasher.update(&server_jar_bytes);
    let computed_hash = format!("{:x}", hasher.finalize());
    
    if computed_hash != version_sha1 {
        return Err(format!("[slapaman] SHA1 hash mismatch: {} != {}", computed_hash, version_sha1));
    }

    Ok(server_jar_bytes.to_vec())
}

pub async fn get_latest_version_id(version_type: VersionType) -> Result<String, String> {
    let manifest = fetch_manifest().await.unwrap();
    let latest_version_id  = match version_type {
        VersionType::Snapshot => manifest["latest"]["snapshot"].as_str().unwrap(),
        VersionType::Release => manifest["latest"]["release"].as_str().unwrap(),
    };

    Ok(latest_version_id.to_string())
}

pub async fn format_version_string(version: &Version) -> String {
    // give the version string proper formatting
    // e.g. "release-latest" -> "release-1.21.5"
    let version_type = match version.v_type {
        VersionType::Release => "release",
        VersionType::Snapshot => "snapshot",
    };
    let version_id = match version.v_id.as_str() {
        "latest" => get_latest_version_id(version.v_type).await.unwrap(),
        _ => version.v_id.clone(),
    };
    let version_string = format!("{}-{}", version_type, version_id);

    version_string
}
