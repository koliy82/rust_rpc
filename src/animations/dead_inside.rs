use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use log::{info, trace};
use tokio::time::sleep;
use crate::models::animation::Animation;

impl Animation {
    pub async fn dead_inside(&self, interval: u64) {
        let mut i = 1000;
        while i > 0 && self.alive_check().await {
            let details = format!("Dead Inside 1000-7: {} - 7 = {}", i, i - 7);
            self.update_discord_activity(&details, "In an endless loop").await;
            sleep(Duration::from_secs(interval)).await;
            i -= 7;
        }
    }
}