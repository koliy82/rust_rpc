use std::{env, thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;
use discord_sdk::{Discord, DiscordApp, Subscriptions};
use discord_sdk::activity::{ActivityBuilder, Assets, Button};
use discord_sdk::user::User;
use discord_sdk::wheel::{UserState, Wheel};
use tokio::task::JoinHandle;
use tokio::{spawn, time};
use tokio::sync::Mutex;
use tracing::{error, info, trace};
use crate::models::animation::Animation;

pub struct Client {
    pub discord: Arc<Mutex<Discord>>,
    pub user: UserState,
    pub wheel: Wheel,
    animation: Option<Animation>,
}

impl Client {
    pub async fn new(app_id: i64, subs: Subscriptions) -> Result<Self, Box<dyn std::error::Error>> {
        let (wheel, handler) = Wheel::new(Box::new(|err| {
            error!(error = ?err, "encountered an error");
        }));

        let mut user = wheel.user();

        let discord = Discord::new(DiscordApp::PlainId(app_id), subs, Box::new(handler))
            .expect("unable to create discord client");

        info!("waiting for handshake...");
        user.0.changed().await.unwrap();

        let user = match &*user.0.borrow() {
            UserState::Connected(user) => user.clone(),
            UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
        };

        let rp = ActivityBuilder::default()
            .details("Fruit Tarts".to_owned())
            .state("Pop Snacks".to_owned())
            .assets(
                Assets::default()
                    .large("the".to_owned(), Some("u mage".to_owned()))
                    .small("the".to_owned(), Some("i mage".to_owned())),
            )
            .button(Button {
                label: "discord-sdk by EmbarkStudios".to_owned(),
                url: "https://github.com/EmbarkStudios/discord-sdk".to_owned(),
            })
            .start_timestamp(SystemTime::now());

        info!("updated activity: {:?}",discord.update_activity(rp).await);
        
        Ok(Self {
            discord: Arc::new(Mutex::new(discord)),
            user: UserState::Connected(user),
            wheel,
            animation: None,
        })
    }
    pub fn start_animation(&mut self, animation_id: i32) {
        if let Some(animation) = &self.animation {
            animation.stop();
        }

        self.animation = Some(Animation::new(self.discord.clone()));

        let animation_clone = match &self.animation {
            None => panic!("Animation is not clonned"),
            Some(value) => value.clone()
        };

        spawn( async move {
            animation_clone.run(animation_id).await;
        });
    }

    pub fn stop_animation(&mut self) {
        if let Some(animation) = &self.animation {
            animation.stop();
            self.animation = None;
        }
    }
}