// Amethyst Launcher
// Copyright 2022-2024 Broken-Deer and contributors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-only

use std::io::Read;

use anyhow::anyhow;
use log::{info, warn};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tauri_plugin_http::reqwest::Client;
use tokio::io::AsyncWriteExt;

use crate::{
    config::instance::InstanceConfig,
    download::Download,
    folder::MinecraftLocation,
    install::vanilla::{generate_assets_downloads, generate_libraries_downloads},
    version::Version,
    HTTP_CLIENT, PLATFORM_INFO,
};

pub async fn complete_files(instance: InstanceConfig, minecraft_location: MinecraftLocation) {
    let assets_lock_file = instance.get_instance_root().join(".aml-assets-ok");
    let libraries_lock_file = instance.get_instance_root().join(".aml-libraries-ok");
    if std::fs::metadata(&assets_lock_file).is_ok() {
        info!("No need to check assets files.");
        info!(
            "helps: you can remove {} file to enable file checking manually.",
            assets_lock_file.display()
        );
    } else {
        info!("Checking and completing assets files");
        complete_assets_files(instance.clone(), minecraft_location.clone()).await;
        info!("Saving assets lock file");
        std::fs::write(assets_lock_file, "ok").unwrap();
    }
    if std::fs::metadata(&libraries_lock_file).is_ok() {
        info!("No need to check libraries files.");
        info!(
            "helps: you can remove {} file to enable file checking manually.",
            libraries_lock_file.display()
        );
    } else {
        info!("Checking and completing libraries files");
        complete_libraries_files(instance.clone(), minecraft_location.clone()).await;
        info!("Saving libraries lock file");
        std::fs::write(libraries_lock_file, "ok").unwrap();
    }
}

pub async fn complete_assets_files(
    instance: InstanceConfig,
    minecraft_location: MinecraftLocation,
) {
    let platform = PLATFORM_INFO.get().unwrap();
    let version =
        Version::from_versions_folder(&minecraft_location, &instance.get_version_id()).unwrap();
    let version = version.parse(&minecraft_location, platform).await.unwrap();
    let assets_downloads =
        generate_assets_downloads(version.asset_index.unwrap(), &minecraft_location)
            .await
            .unwrap();

    let downloads = filter_correct_files(assets_downloads).await;
    if !downloads.is_empty() {
        download_files(downloads).await.unwrap();
    }
}

pub async fn complete_libraries_files(
    instance: InstanceConfig,
    minecraft_location: MinecraftLocation,
) {
    let platform = PLATFORM_INFO.get().unwrap();
    let version =
        Version::from_versions_folder(&minecraft_location, &instance.get_version_id()).unwrap();
    let version = version.parse(&minecraft_location, platform).await.unwrap();
    let library_downloads = generate_libraries_downloads(&version.libraries, &minecraft_location);
    let downloads = filter_correct_files(library_downloads).await;
    if !downloads.is_empty() {
        download_files(downloads).await.unwrap();
    }
}

pub async fn filter_correct_files(downloads: Vec<Download>) -> Vec<Download> {
    downloads
        .into_par_iter()
        .filter(|download| {
            if std::fs::metadata(&download.file).is_err() {
                return true;
            }
            let mut file = match std::fs::File::open(&download.file) {
                Ok(file) => file,
                Err(_) => {
                    return true;
                }
            };
            if download.sha1.is_none() {
                return true;
            };
            let file_hash = calculate_sha1_from_read(&mut file);
            &file_hash != download.sha1.as_ref().unwrap()
        })
        .collect()
}

fn calculate_sha1_from_read<R: Read>(source: &mut R) -> String {
    let mut hasher = sha1_smol::Sha1::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = source.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    hasher.digest().to_string()
}

async fn download_files(downloads: Vec<Download>) -> anyhow::Result<()> {
    let client = HTTP_CLIENT.get().unwrap();
    for download in downloads {
        let mut retried = 0;
        while retried <= 5 {
            retried += 1;
            match download_and_check(client, &download).await {
                Ok(_) => break,
                Err(_) => warn!("Downloaded failed: {}, retried: {}", &download.url, retried),
            }
        }
    }
    Ok(())
}

async fn download_and_check(client: &Client, download: &Download) -> anyhow::Result<()> {
    let file_path = download.file.clone();
    tokio::fs::create_dir_all(file_path.parent().ok_or(anyhow::Error::msg(
        "Unknown Error in instance/mod.rs".to_string(),
    ))?)
    .await?;
    let mut response = client.get(download.url.clone()).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Downloaded failed"));
    }
    let mut file = tokio::fs::File::create(&file_path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.sync_all().await?;
    drop(file);
    let mut file = std::fs::File::open(&file_path).unwrap();
    if let Some(sha1) = download.sha1.clone() {
        if calculate_sha1_from_read(&mut file) != sha1 {
            return Err(anyhow::Error::msg("sha1 check failed".to_string()));
        }
    }
    Ok(())
}