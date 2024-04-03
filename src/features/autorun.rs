use std::env;
use std::path::Path;
use log::{info, trace, warn};
use winreg::enums::{HKEY_CURRENT_USER, RegDisposition};
use winreg::RegKey;

const PROGRAM_KEY_NAME: &str = "koliy82_rust_rpc";

fn autorun_key_get() -> (RegKey, RegDisposition) {
    let current_user = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Run");

    return current_user.create_subkey(&path).unwrap();
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

pub fn autorun_init_check() -> bool{
    let (key, _) = autorun_key_get();

    let current_dir = env::current_exe()
        .expect("Failed find current execute file directory.");

    let path = current_dir.as_os_str().to_str()
        .expect("Failed to parse file directory.");

    match key.get_value::<String, &str>(PROGRAM_KEY_NAME) {
        Ok(value) => {
            if !value.eq(path) {
                key.set_value(PROGRAM_KEY_NAME, &path).unwrap();
                warn!("Autorun path is different, path updated.");
            }
            true
        }
        Err(_) => {
            false
        }
    }
}