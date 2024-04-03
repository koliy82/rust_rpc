use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, SystemTime};
use discord_sdk::activity::{ActivityBuilder, Assets, Button};
use log::{info, trace};
use serde::de::Unexpected::Str;
use tokio::time::sleep;
use crate::models::animation::Animation;

impl Animation {
    pub async fn dead_inside(&self, interval: u64, started_time: SystemTime) {
            let mut i = 1000;
            while self.alive_check().await {
                let state = format!("{} - 7 = {}", i, i - 7);
                let details = "Dead Inside 1000-7";

                let activity = ActivityBuilder::default()
                    .details(details)
                    .state(state)
                    .assets(
                        Assets::default()
                            .large("dota", Some(""))
                            .small("the", Some("")),
                    ).start_timestamp(started_time);

                self.update_discord_activity(activity).await;
                sleep(Duration::from_secs(interval)).await;
                i -= 7;
                if i < 0{
                    i = 1000;
                }
            }
        }
    
}