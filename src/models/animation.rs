use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{env, thread};
use std::time::{Duration, SystemTime};
use discord_sdk::activity::{Activity, ActivityBuilder, Assets, Button};
use discord_sdk::Discord;
use discord_sdk::lobby::search::LobbySearchCast::String;
use log::trace;
use ringbuffer::AllocRingBuffer;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use crate::models::client::Client;

#[derive(Clone)]
pub struct Animation {
    token: Arc<CancellationToken>,
    discord: Arc<Mutex<Discord>>,
}

impl Animation {
    pub fn new(discord: Arc<Mutex<Discord>>) -> Self {
        Self {
            token: Arc::new(CancellationToken::new()),
            discord,
        }
    }

    pub async fn run(&self, animation_id: i32) {
        let interval = match env::var("INTERVAL_SECONDS") {
            Ok(value) => value.parse::<u64>().unwrap(),
            Err(_) => 5,
        };
        info!("Animation №{} is starting...", &animation_id);
        match animation_id {
            1 => self.dead_inside(interval).await,
            2 => self.loading(interval).await,
            3 => self.json_custom(interval).await,
            _ => error!("Неверный ID анимации"),
        }
    }

    pub fn stop(&self) {
        info!("animation is stopping... {:?}", self.token);
        self.token.cancel();
    }

    pub(crate) async fn update_discord_activity(&self, details: &str, state: &str) {
        let client = self.discord.lock().await;
        let update = client.update_activity(
            ActivityBuilder::default()
            .details(details)
            .state(state)
        ).await;
        info!("updated activity: {:?}", update);
    }

    pub async fn alive_check(&self) -> bool {
        match self.token.is_cancelled() {
            true => {
                self.discord.lock().await.clear_activity().await;
                false
            }
            false => true
        }
    }

}
