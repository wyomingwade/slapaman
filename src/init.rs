use anyhow::{Context, Result};
use directories::ProjectDirs;
use regex::Regex;
use std::fs::create_dir_all;
use std::fs::File;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn slapaman_init() -> Result<(), String> {
    ensure_slapaman_dir_exists().unwrap();
    ensure_java_is_installed().unwrap();
    ensure_slapaman_server_list_exists().unwrap();

    Ok(())
}

pub fn ensure_slapaman_dir_exists() -> Result<PathBuf> {
    let proj = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .context("could not determine a home directory")?;

    let dir = proj.data_dir(); // Path: &Path
    create_dir_all(dir).with_context(|| format!("failed to create {}", dir.display()))?;
    Ok(dir.to_path_buf())
}

fn ensure_slapaman_server_list_exists() -> Result<PathBuf> {
    let proj = ProjectDirs::from("com", "wyomingwade", "slapaman")
        .context("could not determine a home directory")?;

    let file = proj.data_dir().join("servers.lock");
    if !file.exists() {
        File::create(&file).with_context(|| format!("failed to create {}", &file.display()))?;
    }
    Ok(file.to_path_buf())
}

fn ensure_java_is_installed() -> Result<(), String> {
    let min_version = 8;
    let java_bin = locate_java(min_version).unwrap();

    if !java_bin.exists() {
        return Err(format!(
            "[slapaman] java is not installed: {}",
            java_bin.display()
        ));
    }

    Ok(())
}

fn locate_java(min_major: u32) -> Result<PathBuf> {
    // JAVA_HOME
    if let Some(home) = env::var_os("JAVA_HOME") {
        let bin = Path::new(&home).join(if cfg!(windows) {
            "bin\\java.exe"
        } else {
            "bin/java"
        });
        if bin.exists() && version_ok(&bin, min_major)? {
            return Ok(bin);
        }
    }

    // 2$PATH  (uses the `which` crate)
    if let Ok(path) = which::which("java") {
        if version_ok(&path, min_major)? {
            return Ok(path);
        }
    }

    anyhow::bail!(
        "[slapaman] java {} not found on this system. \
                   install an OpenJDK distribution (e.g. Temurin) \
                   and ensure it is on PATH or set JAVA_HOME.",
        min_major
    )
}

fn version_ok(java_bin: &Path, min_major: u32) -> Result<bool> {
    let out = Command::new(java_bin)
        .arg("-version")
        .output()
        .with_context(|| format!("failed to execute {:?}", java_bin))?;

    // `java -version` prints to *stderr*.
    let text = String::from_utf8_lossy(&out.stderr);

    // works for all modern formats: 21, 17.0.10, "1.8.0_402", etc.
    let re = Regex::new(r#"version "(?:(\d+)\.)?(\d+)"#).unwrap();
    let caps = re
        .captures(&text)
        .context("couldn’t parse java -version output")?;

    let major = caps
        .get(1)
        .or_else(|| caps.get(2)) // Java 8 style “1.8”
        .unwrap()
        .as_str()
        .parse::<u32>()?;

    Ok(major >= min_major)
}
