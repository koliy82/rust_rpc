mod animations;

use std::{env, thread};
use std::error::Error;
use std::ops::Not;
use std::path::Path;
use std::time::Duration;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use dotenv::dotenv;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
use log::{error, info, trace, warn};
use crate::animations::Animations;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Trace).unwrap();
    info!("Starting Discord Rich Presence...");
    dotenv().ok();
    autorun_init();

    let client_id = env::var("CLIENT_ID").expect("Not find CLIENT_ID in .env");
    let animation_id = match env::var("ANIMATION_ID") {
        Ok(value) => value.parse::<i32>().unwrap(),
        Err(_) => 1,
    };

    let mut client = DiscordIpcClient::new(client_id.as_str()).expect("Failed to create client");

    loop {
        match client.connect() {
            Ok(_) => {
                info!("Client connected to Discord successfully.");
                break;
            }
            Err(_) => {
                error!("Client failed to connect to Discord, next try...");
            }
        };
        thread::sleep(Duration::from_secs(10));
    }


    loop {
        Animations::run(animation_id, &mut client);
        trace!("animation end");
        thread::sleep(Duration::from_secs(10));
    }
}

fn autorun_init() {
    match env::var("AUTORUN_WINDOWS") {
        Ok(is_run) => {
            let current_user = RegKey::predef(HKEY_CURRENT_USER);
            let path = Path::new("Software").join("Microsoft").join("Windows").join("CurrentVersion").join("Run");
            let (key, _) = current_user.create_subkey(&path).unwrap();

            if is_run == "true" {
                let current_dir = env::current_exe().expect("Failed find current execute file directory.");
                let path = current_dir.as_os_str().to_str().expect("Failed to parse file directory.");
                match key.get_value::<String, &str>("test") {
                    Ok(value) => {
                        if value.eq(path).not() {
                            warn!("Autorun path is different, updating...");
                            key.set_value("test", &path).unwrap();
                        }
                    }
                    Err(_) => {
                        key.set_value("test", &path).unwrap();
                    }
                }
            } else {
                key.delete_value("test").unwrap();
            }
        }
        Err(_) => { return; }
    }
}