use std::env;
use std::time::{Duration, SystemTime};
use discord_sdk::activity::{ActivityBuilder, Assets};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::de::Unexpected::Str;
use tokio::time::sleep;
use crate::models::animation;
use crate::models::animation::Animation;
use crate::models::json_animation::CustomAnimations;

impl Animation {
    pub async fn json_custom(&self, interval: u64, started_time: SystemTime) {
        let file_name = env::var("CUSTOM_ANIMATION_FILENAME").unwrap_or_else(|_| String::from("custom_animation"));
        let animation = CustomAnimations::parse(file_name).await;
        
        let mut animations = animation.animations.clone();
        if animation.randomize.unwrap_or(false) {
            animations.shuffle(&mut thread_rng());
        }

        loop {
            for animation in animations.clone() {
                if !self.alive_check().await {
                    return;
                }
                println!("{:?}", animation);

                let state = animation.state.unwrap_or_else(|| String::from("custom_animation_state"));
                let details = animation.details.unwrap_or_else(|| String::from("custom_animation_details"));
                let large_image_text = animation.large_image_text.unwrap_or_else(|| String::from(""));
                let large_image_key = animation.large_image_key.unwrap_or_else(|| String::from("dota"));
                let small_image_text = animation.small_image_text.unwrap_or_else(|| String::from(""));
                let small_image_key = animation.small_image_key.unwrap_or_else(|| String::from("the"));


                let activity = ActivityBuilder::default()
                    .details(details)
                    .state(state)
                    .start_timestamp(started_time)
                    .assets(
                        Assets::default()
                            .large(large_image_key, Some(large_image_text))
                            .small(small_image_key, Some(small_image_text))
                    );

                self.update_discord_activity(activity).await;
                sleep(Duration::from_secs(interval)).await;
            }
        }

    }
}