use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, SystemTime};
use discord_sdk::activity::ActivityBuilder;
use log::{info, trace};
use ringbuffer::AllocRingBuffer;
use crate::models::animation::Animation;

impl Animation {
    pub async fn loading(&self, interval: u64, started_time: SystemTime) {
        info!("Starting Loading Animation");
        let started = SystemTime::now();
        
        let animations_char: Vec<&str> = vec![
            "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▁",
        ];
        let mut buffer = std::string::String::from("");

        for element in animations_char {
            if self.alive_check().await{
                break;
            }
            buffer.push_str(element);
            let formatted_str = format!("{} load", element.clone());
            
            let state = "Loading...";
            let details = &buffer;

            let activity = ActivityBuilder::default()
                .details(details)
                .state(state)
                .start_timestamp(started_time);

            self.update_discord_activity(activity).await;
            trace!("{}", element);
            tokio::time::sleep(Duration::from_secs(interval)).await;
        }
    }
}
