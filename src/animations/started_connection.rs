use std::time::{Duration, SystemTime};
use discord_sdk::activity::{ActivityBuilder, Assets, Button};
use tokio::time::sleep;
use crate::models::animation::Animation;

impl Animation {
    pub async fn started_connection(&self, interval: u64, started_time: SystemTime) {
        let animations_char: Vec<&str> = vec![
            "Click tray to load animation", 
            "Animations by koliy82"
        ];
        while self.alive_check().await {
            let details = animations_char.first().unwrap().to_string();
            
            let activity = ActivityBuilder::default()
                .details(details)
                .state("Click tray to load animation")
                .assets(
                    Assets::default()
                        .large("dota", Some(""))
                )
                .button(Button {
                    label: "Create You Animation by koliy82".to_owned(),
                    url: "https://github.com/koliy82/rust_rpc".to_owned(),
                })
                .start_timestamp(started_time);

            self.update_discord_activity(activity).await;
            sleep(Duration::from_secs(interval)).await;
        }
    }
    
}