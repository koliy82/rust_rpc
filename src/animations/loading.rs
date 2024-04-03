use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use log::{info, trace};
use ringbuffer::AllocRingBuffer;
use crate::models::animation::Animation;

impl Animation {
    pub async fn loading(&self, interval: u64) {
        info!("Starting Loading Animation");

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
            self.update_discord_activity(&buffer, "In an endless loop").await;
            trace!("{}", element);
            tokio::time::sleep(Duration::from_secs(interval)).await;
        }
    }
}
