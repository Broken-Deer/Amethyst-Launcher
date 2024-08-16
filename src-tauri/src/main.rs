// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod config;
pub mod download;
pub mod folder;
pub mod game_data;
pub mod install;
pub mod instance;
pub mod platform;
pub mod utils;
pub mod version;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::get_user_config;
use crate::game_data::{scan_mod_folder, scan_saves_folder};
use crate::install::install;
use crate::install::{
    get_fabric_version_list, get_forge_version_list, get_minecraft_version_list,
    get_quilt_version_list,
};
use crate::instance::{
    check_repeated_instance_name, create_instance, get_instance_config,
    get_instance_config_from_string, scan_instances_folder, set_current_instance,
};
use folder::DataLocation;
use once_cell::sync::OnceCell;
use platform::{OsType, PlatformInfo};
use tauri::{Listener, Manager, Window};
use tauri_plugin_http::reqwest;

/// use MAIN_WINDOW.emit() to send message to main window
static MAIN_WINDOW: OnceCell<Window> = OnceCell::new();
static DATA_LOCATION: OnceCell<DataLocation> = OnceCell::new();
static PLATFORM_INFO: OnceCell<PlatformInfo> = OnceCell::new();
static HTTP_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();
static APPLICATION_DATA: OnceCell<PathBuf> = OnceCell::new();
static DEFAULT_LAUNCHER_PROFILE: &[u8] = include_bytes!("./launcher_profiles.json");

pub struct Storage {
    pub current_instance: Arc<Mutex<String>>,
}
#[tokio::main]
async fn main() {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    tokio::spawn(initialize_application());
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_user_config,
            create_instance,
            get_minecraft_version_list,
            get_fabric_version_list,
            get_forge_version_list,
            get_quilt_version_list,
            check_repeated_instance_name,
            scan_instances_folder,
            // watch_instances_folder,
            set_current_instance,
            scan_mod_folder,
            scan_saves_folder,
            get_instance_config,
            get_instance_config_from_string,
            install,
        ])
        .manage(Storage {
            current_instance: Arc::new(Mutex::new("".to_string())),
        })
        .setup(move |app| {
            MAIN_WINDOW.set(app.get_window("main").unwrap()).unwrap();
            app.listen_any("fontend-loaded", move |_| {
                println!(
                    "
 █████╗ ███╗   ███╗███████╗████████╗██╗  ██╗██╗   ██╗███████╗████████╗    
██╔══██╗████╗ ████║██╔════╝╚══██╔══╝██║  ██║╚██╗ ██╔╝██╔════╝╚══██╔══╝    
███████║██╔████╔██║█████╗     ██║   ███████║ ╚████╔╝ ███████╗   ██║       
██╔══██║██║╚██╔╝██║██╔══╝     ██║   ██╔══██║  ╚██╔╝  ╚════██║   ██║       
██║  ██║██║ ╚═╝ ██║███████╗   ██║   ██║  ██║   ██║   ███████║   ██║       
╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚══════╝   ╚═╝       
                                                                          
██╗      █████╗ ██╗   ██╗███╗   ██╗ ██████╗██╗  ██╗███████╗██████╗        
██║     ██╔══██╗██║   ██║████╗  ██║██╔════╝██║  ██║██╔════╝██╔══██╗       
██║     ███████║██║   ██║██╔██╗ ██║██║     ███████║█████╗  ██████╔╝       
██║     ██╔══██║██║   ██║██║╚██╗██║██║     ██╔══██║██╔══╝  ██╔══██╗       
███████╗██║  ██║╚██████╔╝██║ ╚████║╚██████╗██║  ██║███████╗██║  ██║       
╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝       
             "
                )
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_application() {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    PLATFORM_INFO.set(PlatformInfo::new().await).unwrap();
    initialize_application_data().await;
    DATA_LOCATION
        .set(DataLocation::new(APPLICATION_DATA.get().unwrap()))
        .unwrap();
    HTTP_CLIENT
        .set(
            reqwest::ClientBuilder::new()
                .pool_idle_timeout(Duration::from_secs(30))
                .pool_max_idle_per_host(1)
                .build()
                .unwrap(),
        )
        .unwrap();
    let launch_profiles_path = DATA_LOCATION
        .get()
        .unwrap()
        .root
        .join("launcher_profiles.json");
    let _ = tokio::fs::remove_file(&launch_profiles_path).await;
    tokio::fs::write(&launch_profiles_path, DEFAULT_LAUNCHER_PROFILE)
        .await
        .expect("C");
    instance::update_latest_instance().await;
}

async fn initialize_application_data() {
    let platform_info = PLATFORM_INFO.get().unwrap();
    match platform_info.os_type {
        OsType::Windows => {
            APPLICATION_DATA
                .set(
                    PathBuf::from(
                        std::env::var("APP_DATA").expect("Could not found APP_DATA directory"),
                    )
                    .join("aml"),
                )
                .unwrap();
        }
        OsType::Linux => {
            APPLICATION_DATA
                .set(
                    PathBuf::from(std::env::var("HOME").expect("Could not found home"))
                        .join(".aml"),
                )
                .unwrap();
        }
        OsType::Osx => {
            APPLICATION_DATA
                .set(PathBuf::from("/Users/").join("aml"))
                .unwrap();
        }
    }
}
