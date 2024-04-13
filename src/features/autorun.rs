use std::env;
use std::ops::Not;
use std::path::Path;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use cfg_if::cfg_if;
use log::{error, info, trace, warn};
use once_cell::sync::Lazy;
use serde_json::Value::Bool;
use tokio::sync::Mutex;
use tracing::Instrument;
cfg_if::cfg_if! {
    if #[cfg(windows)] {
        use winreg::enums::{HKEY_CURRENT_USER, RegDisposition};
        use winreg::RegKey;
    } else {

    }
}
use crate::{features, init};

pub(crate) const PROGRAM_KEY_NAME: &str = "Koliy82RPC";
const AUTOSTART_SUB_KEY: &str = "autostart";
pub static AUTORUN: Lazy<Mutex<AutoLaunch>> = Lazy::new(|| Mutex::new(
    AutoLaunchBuilder::new()
        .set_app_name(PROGRAM_KEY_NAME)
        .set_app_path(
            env::current_exe()
                .expect("Failed find current execute file directory.")
                .as_os_str().to_str().expect("Failed to parse file directory.")
        )
        .set_use_launch_agent(true)
        .build()
        .unwrap()
    )
);

#[cfg(target_os = "windows")]
fn autorun_key_get() -> (RegKey, RegDisposition) {
    return RegKey::predef(HKEY_CURRENT_USER).create_subkey(
        Path::new("Software")
          .join("Microsoft")
          .join("Windows")
          .join("CurrentVersion")
          .join("Run")
    ).unwrap();
}

#[cfg(target_os = "windows")]
fn program_key_get() -> (RegKey, RegDisposition) {
    return RegKey::predef(HKEY_CURRENT_USER).create_subkey(
        Path::new("Software")
            .join(PROGRAM_KEY_NAME)
    ).unwrap();
}


pub fn autorun_change(is_set: bool) {
    // let (key, _) = autorun_key_get();
    
    let launch = AUTORUN.try_lock().expect("aa");

    if(!is_set){
        // key.delete_value(PROGRAM_KEY_NAME).unwrap();
        launch.disable().unwrap();
        info!("Autorun is removed.");
        return;
    }

    let current_dir = env::current_exe()
        .expect("Failed find current execute file directory.");
    
    let path = current_dir.as_os_str().to_str()
        .expect("Failed to parse file directory.");

    match launch.clone().is_enabled() {
        Ok(is_enabled) => { 
            if is_enabled {
                warn!("Autorun path already is added")
            } else{
                launch.enable().unwrap();
                info!("Autorun path added.")
            }
        }
        Err(_) => {
            launch.enable().unwrap();
            info!("Autorun path added.")
        }
    }

    // match key.get_value::<String, &str>(PROGRAM_KEY_NAME) {
    //     Ok(value) => { warn!("Autorun path already is added") }
    //     Err(_) => {
    //         key.set_value(PROGRAM_KEY_NAME, &path).unwrap();
    //         info!("Autorun path added.")
    //     }
    // }

}

#[cfg(target_os = "windows")]
pub fn autostart_change() {
    
    let (key, _) = program_key_get();
    match key.get_value::<String, &str>(AUTOSTART_SUB_KEY) {
        Ok(value) => {
            key.set_value(AUTOSTART_SUB_KEY, &(!value.parse::<bool>().expect("failed to parse program starting boolean value")).to_string()).unwrap();
            warn!("Autostart path changed") 
        }
        Err(_) => {
            key.set_value(AUTOSTART_SUB_KEY, &"false").unwrap();
            error!("Autostart path is not added.")
        }
    }
    
}

#[cfg(target_os = "windows")]
pub async fn reg_init_check() -> (bool, bool){

    let launch = AUTORUN.lock().await;
    
    // let (autorun_key, _) = autorun_key_get();
    // let current_dir = env::current_exe()
    //     .expect("Failed find current execute file directory.");
    // let path = current_dir.as_os_str().to_str()
    //     .expect("Failed to parse file directory.");
    
    // let is_autorun = match autorun_key.get_value::<String, &str>(PROGRAM_KEY_NAME) {
    //     Ok(value) => {
    //         if !value.eq(path) {
    //             autorun_key.set_value(PROGRAM_KEY_NAME, &path).unwrap();
    //             warn!("Autorun path is different, path updated.");
    //         }
    //         true
    //     }
    //     Err(_) => {
    //         false
    //     }
    // };

    let is_autorun = match launch.clone().is_enabled() {
        Ok(value) => {
            let (autorun_key, _) = autorun_key_get();
            if let Ok(value) = autorun_key.get_value::<String, &str>(PROGRAM_KEY_NAME) {
                info!("{}" , launch.get_app_path());
                info!("{}" , value);
                info!("{}" , !launch.get_app_path().eq(&value));
                if !launch.get_app_path().eq(&value) {
                    launch.enable();
                    warn!("Autorun path is different, path updated.");
                }
            };
            // if !value.eq(path) {
            //     autorun_key.set_value(PROGRAM_KEY_NAME, &path).unwrap();
            //     warn!("Autorun path is different, path updated.");
            // }
            value
        }
        Err(_) => {
            false
        }
    };

    let (program_key, _) = program_key_get();
    
    let is_starting = match program_key.get_value::<String, &str>(AUTOSTART_SUB_KEY) {
        Ok(value) => value.parse().expect("failed to parse program starting boolean value"),
        Err(e) => {
            program_key.set_value(AUTOSTART_SUB_KEY, &"false");
            false
        }
    };

    (is_autorun, is_starting)
}