#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::error::Error;
use std::ops::{Not};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};
use std::{env, thread};
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread::{sleep, spawn};
use discord_sdk::activity::{ActivityBuilder, Assets, Button};
use discord_sdk::Subscriptions;

use crate::models::animation::Animation;
use dotenv::dotenv;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tracing::{info, Level, trace};
use tray_icon::menu::{AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem};
use tray_icon::{TrayIconBuilder, TrayIconEvent};
use tray_icon::menu::MenuItemKind::Check;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
use crate::features::autorun::{autorun_change, autostart_change, reg_init_check};
use crate::features::utils::load_icon;
use crate::models::client::{Client};
use crate::models::json_animation::CustomAnimations;

mod models;
mod animations;
mod features;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::TRACE)
        .init();
    trace!("Initialise program...");

    dotenv().ok();
    tray_start().await;

    Ok(())
}

async fn tray_start() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icons/program.ico");
    let icon = load_icon(Path::new(path));
    let (is_autorun, is_program_started) = reg_init_check();
    
    let event_loop = EventLoopBuilder::new().build();
    let tray_menu = Menu::new();

    let enable_i = CheckMenuItem::new("DiscordRPC by koliy82", true, is_program_started, None);
    let autorun_i = CheckMenuItem::new("Autorun", true, is_autorun, None);
    let quit_i = MenuItem::new("Quit", true, None);

    tray_menu.append_items(&[
        &enable_i,
        &autorun_i,
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("rust rpc".to_string()),
                copyright: Some("Copyright".to_string()),
                ..Default::default()
            }),
        ),
        &quit_i,
    ]);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("rust_rpc")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    
    let app_id = match env::var("APP_ID") {
        Ok(value) => value.parse::<i64>().unwrap(),
        Err(_) => 1225132045596885105,
    };
    
    let mut client = match Client::new(app_id).await {
        Ok(value) => value,
        Err(_) => {
            panic!("EXCEPTION ON CREATE DISCORD CLIENT")
        }
    };

    if is_program_started{
        let animation_id = match env::var("ANIMATION_ID") {
            Ok(value) => value.parse::<i32>().unwrap(),
            Err(_) => 1,
        };
        client.start_animation(animation_id);
    }

    event_loop.run( move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = menu_channel.try_recv() {
            
            if event.id == enable_i.id() {
                autostart_change();
                if enable_i.is_checked() {
                    trace!("on");
                    let animation_id = match env::var("ANIMATION_ID") {
                        Ok(value) => value.parse::<i32>().unwrap(),
                        Err(_) => 1,
                    };
                    client.start_animation(animation_id);
                }else{
                    trace!("off");
                    client.stop_animation();
                }
            }
            if event.id == autorun_i.id() {
                autorun_change(autorun_i.is_checked())
            }
            if event.id == quit_i.id() {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
            trace!("{event:?}");
        }

        if let Ok(event) = tray_channel.try_recv() {
            trace!("{event:?}");
        }
    })
}