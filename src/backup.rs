// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use fs_extra::{copy_items, dir::CopyOptions};

use crate::server::Server;
use crate::world::set_world;

const WORLD_DIR_NAME: &str = "world";
const BACKUPS_DIR_NAME: &str = "backups";
const LEVEL_DAT: &str = "level.dat";

pub fn create_world_backup(
    verbose: u8,
    name: String,
    tag: Option<String>,
) -> Result<PathBuf, String> {
    let server = Server::load_by_name(&name)?;
    let server_dir = server.path.join(&name);

    if !server_dir.exists() {
        return Err(format!("server instance does not exist: {}", name));
    }

    let world_dir = server_dir.join(WORLD_DIR_NAME);
    validate_world_root(&world_dir, "world")?;

    let backups_dir = server_dir.join(BACKUPS_DIR_NAME);
    fs::create_dir_all(&backups_dir)
        .map_err(|e| format!("failed to create backups directory: {}", e))?;

    let backup_name = build_backup_name(&tag);
    let backup_path = unique_backup_path(&backups_dir, &backup_name);
    fs::create_dir_all(&backup_path)
        .map_err(|e| format!("failed to create backup directory: {}", e))?;

    let mut world_contents = Vec::new();
    for entry in world_dir
        .read_dir()
        .map_err(|e| format!("failed to read world directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("failed to read world entry: {}", e))?;
        world_contents.push(entry.path());
    }

    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64_000,
        copy_inside: true,
        content_only: false,
        depth: 0,
    };

    copy_items(&world_contents, &backup_path, &options)
        .map_err(|e| format!("failed to copy world contents: {}", e))?;

    if verbose > 0 {
        println!(
            "[slapaman] created backup for {} at {}",
            name,
            backup_path.display()
        );
    }

    Ok(backup_path)
}

pub fn restore_world_backup(verbose: u8, name: String, backup: &PathBuf) -> Result<(), String> {
    let server = Server::load_by_name(&name)?;
    let server_dir = server.path.join(&name);

    if !server_dir.exists() {
        return Err(format!("server instance does not exist: {}", name));
    }

    let backups_dir = server_dir.join(BACKUPS_DIR_NAME);
    let resolved_backup = resolve_backup_path(&backups_dir, backup)?;
    validate_world_root(&resolved_backup, "backup")?;

    set_world(verbose, name, &resolved_backup)
}

fn validate_world_root(path: &Path, label: &str) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("{} does not exist: {}", label, path.display()));
    }
    if !path.is_dir() {
        return Err(format!("{} is not a directory: {}", label, path.display()));
    }
    if !path.join(LEVEL_DAT).exists() {
        return Err(format!(
            "{} is missing {}: {}",
            label,
            LEVEL_DAT,
            path.display()
        ));
    }

    Ok(())
}

fn build_backup_name(tag: &Option<String>) -> String {
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let mut name = format!("world-{}", timestamp);

    if let Some(tag) = tag {
        let cleaned = sanitize_tag(tag);
        if !cleaned.is_empty() {
            name.push('-');
            name.push_str(&cleaned);
        }
    }

    name
}

fn unique_backup_path(backups_dir: &Path, base_name: &str) -> PathBuf {
    let mut candidate = backups_dir.join(base_name);
    let mut index = 1;

    while candidate.exists() {
        candidate = backups_dir.join(format!("{}-{:02}", base_name, index));
        index += 1;
    }

    candidate
}

fn resolve_backup_path(backups_dir: &Path, backup: &PathBuf) -> Result<PathBuf, String> {
    if backup.as_path().exists() {
        return Ok(backup.clone());
    }

    let candidate = backups_dir.join(backup);
    if candidate.exists() {
        return Ok(candidate);
    }

    Err(format!("backup not found: {}", backup.display()))
}

fn sanitize_tag(tag: &str) -> String {
    tag.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' => '-',
            _ => '_',
        })
        .collect::<String>()
        .trim_matches(&['-', '_'][..])
        .to_string()
}
