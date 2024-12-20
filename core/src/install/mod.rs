// Amethyst Launcher
// Copyright 2022-2024 Broken-Deer and contributors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-only

use forge::version_list::ForgeVersionList;
use log::{debug, error, info};
use quilt::QuiltVersionList;
use tauri::Emitter;
use tokio::io::AsyncWriteExt;
use vanilla::generate_download_info;

use crate::{
    config::instance::{InstanceRuntime, ModLoaderType},
    download::{download_files, Progress, ProgressError},
    folder::{DataLocation, MinecraftLocation},
    instance::Instance,
    version::VersionManifest,
    Storage, DATA_LOCATION, MAIN_WINDOW,
};

mod fabric;
mod forge;
mod quilt;
pub mod vanilla;

#[tauri::command(async)]
pub async fn get_minecraft_version_list() -> Option<VersionManifest> {
    VersionManifest::new().await.ok()
}

#[tauri::command(async)]
pub async fn get_fabric_version_list(mcversion: String) -> Option<fabric::LoaderArtifactList> {
    fabric::LoaderArtifactList::new(&mcversion).await.ok()
}

#[tauri::command(async)]
pub async fn get_forge_version_list(mcversion: String) -> Option<ForgeVersionList> /* Option<ForgeVersionList> */
{
    ForgeVersionList::new(&mcversion).await.ok()
}

#[tauri::command(async)]
pub async fn get_quilt_version_list(mcversion: String) -> Option<QuiltVersionList> {
    QuiltVersionList::new(&mcversion).await.ok()
}

#[tauri::command(async)]
pub async fn install(
    storage: tauri::State<'_, Storage>,
    instance: Instance,
) -> std::result::Result<(), ()> {
    let main_window = MAIN_WINDOW.get().unwrap();
    main_window
        .emit(
            "install_progress",
            Progress {
                completed: 0,
                total: 0,
                step: 1, // Step 1 is get version info
            },
        )
        .unwrap();
    info!(
        "Start installing the game for instance {}",
        instance.config.name
    );
    #[allow(clippy::unwrap_used)]
    let data_location = DATA_LOCATION.get().unwrap();
    let runtime = instance.config.runtime;
    info!("------------- Instance runtime config -------------");
    info!("-> Minecraft: {}", runtime.minecraft);
    match &runtime.mod_loader_type {
        Some(x) => info!("-> Mod loader: {x}"),
        None => info!("-> Mod loader: none"),
    };
    match &runtime.mod_loader_version {
        Some(x) => info!("-> Mod loader version: {x}"),
        None => info!("-> Mod loader version: none"),
    };
    info!("Generating download task...");
    let download_list = match generate_download_info(
        &runtime.minecraft,
        MinecraftLocation::new(&data_location.root),
    )
    .await
    {
        Ok(x) => x,
        Err(_) => {
            main_window
                .emit("install_error", ProgressError { step: 1 })
                .unwrap();
            return Err(());
        }
    };
    info!("Start downloading file");
    let config = storage.config.lock().unwrap().clone();
    download_files(
        download_list,
        true,
        true,
        config.download.max_connection,
        config.download.max_download_speed,
    )
    .await;
    if runtime.mod_loader_type.is_some() {
        info!("Install mod loader");
        main_window
            .emit(
                "install_progress",
                Progress {
                    completed: 0,
                    total: 0,
                    step: 4,
                },
            )
            .unwrap();
        match install_mod_loader(runtime, data_location).await {
            Ok(_) => (),
            Err(_) => {
                error!("Failed to install mod loader");
                main_window
                    .emit("install_error", ProgressError { step: 4 })
                    .unwrap();
                return Err(());
            }
        };
    }
    debug!("Saving lock file");
    let mut lock_file = tokio::fs::File::create(
        data_location
            .get_instance_root(&instance.config.name)
            .join(".install.lock"),
    )
    .await
    .unwrap();
    lock_file.write_all(b"ok").await.unwrap();
    main_window.emit("install_success", "").unwrap();
    Ok(())
}

async fn install_mod_loader(
    runtime: InstanceRuntime,
    data_location: &DataLocation,
) -> anyhow::Result<()> {
    let mod_loader_type = runtime.mod_loader_type.unwrap();
    let mod_loader_version = runtime
        .mod_loader_version
        .ok_or(anyhow::Error::msg("bad instance.toml file!"))?;
    match mod_loader_type {
        ModLoaderType::Fabric => {
            fabric::install(
                &runtime.minecraft,
                &mod_loader_version,
                MinecraftLocation::new(&data_location.root),
            )
            .await?
        }
        ModLoaderType::Quilt => {
            quilt::install(
                &runtime.minecraft,
                &mod_loader_version,
                MinecraftLocation::new(&data_location.root),
            )
            .await?
        }
        ModLoaderType::Forge => {
            forge::install(
                &DATA_LOCATION.get().unwrap().root,
                &mod_loader_version,
                &runtime.minecraft,
            )
            .await?
        }
        ModLoaderType::Neoforge => {}
    }

    anyhow::Ok(())
}
