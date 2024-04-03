use std::time::Duration;
use tokio::time::sleep;
use crate::models::animation::Animation;

impl Animation {
    pub async fn json_custom(&self, interval: u64) {
        // TODO parse json animation
    }
}