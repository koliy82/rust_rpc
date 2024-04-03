use std::env;
use std::path::Path;
use log::{error, info, trace, warn};
use serde_json::Value::Bool;
use winreg::enums::{HKEY_CURRENT_USER, RegDisposition};
use winreg::RegKey;

const PROGRAM_KEY_NAME: &str = "Koliy82RPC";
const AUTOSTART_SUB_KEY: &str = "autostart";

fn autorun_key_get() -> (RegKey, RegDisposition) {
    return RegKey::predef(HKEY_CURRENT_USER).create_subkey(
        Path::new("Software")
          .join("Microsoft")
          .join("Windows")
          .join("CurrentVersion")
          .join("Run")
    ).unwrap();
}

fn program_key_get() -> (RegKey, RegDisposition) {
    return RegKey::predef(HKEY_CURRENT_USER).create_subkey(
        Path::new("Software")
            .join(PROGRAM_KEY_NAME)
    ).unwrap();
}

pub fn autorun_change(is_set: bool) {

    let (key, _) = autorun_key_get();

    if(!is_set){
        key.delete_value(PROGRAM_KEY_NAME).unwrap();
        info!("Autorun path removed.");
        return;
    }

    let current_dir = env::current_exe()
        .expect("Failed find current execute file directory.");

    let path = current_dir.as_os_str().to_str()
        .expect("Failed to parse file directory.");

    match key.get_value::<String, &str>(PROGRAM_KEY_NAME) {
        Ok(value) => { warn!("Autorun path already is added") }
        Err(_) => {
            key.set_value(PROGRAM_KEY_NAME, &path).unwrap();
            info!("Autorun path added.")
        }
    }

}

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

pub fn reg_init_check() -> (bool, bool){
    let (autorun_key, _) = autorun_key_get();

    let current_dir = env::current_exe()
        .expect("Failed find current execute file directory.");
    let path = current_dir.as_os_str().to_str()
        .expect("Failed to parse file directory.");

    let is_autorun = match autorun_key.get_value::<String, &str>(PROGRAM_KEY_NAME) {
        Ok(value) => {
            if !value.eq(path) {
                autorun_key.set_value(PROGRAM_KEY_NAME, &path).unwrap();
                warn!("Autorun path is different, path updated.");
            }
            true
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