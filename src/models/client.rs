use std::{env, thread};
use std::ops::Add;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};
use discord_sdk::{Discord, DiscordApp, Subscriptions};
use discord_sdk::activity::{ActivityBuilder, Assets, Button, IntoTimestamp};
use discord_sdk::user::User;
use discord_sdk::wheel::{UserState, Wheel};
use log::Log;
use tokio::task::JoinHandle;
use tokio::{spawn, time};
use tokio::sync::Mutex;
use tokio::sync::watch::error::RecvError;
use tracing::{error, info, trace};
use crate::models::animation::Animation;

pub struct Client {
    app_id: i64,
    subscriptions: Subscriptions,
    pub discord: Arc<Mutex<Discord>>,
    pub user: User,
    pub wheel: Wheel,
    animation: Option<Animation>,
    pub started_time: SystemTime,
}

impl Client {
    pub async fn new(app_id: i64) -> Result<Self, Box<dyn std::error::Error>> {
        let started_time = SystemTime::now();
        let subscriptions = Subscriptions::empty();

        let (wheel, handler) = Wheel::new(Box::new(|err| {
            error!(error = ?err, "encountered an error");
        }));

        let mut user = wheel.user();
        
        let discord = Discord::new(DiscordApp::PlainId(app_id), subscriptions, Box::new(handler))
            .expect("unable to create discord client");

        info!("waiting for handshake...");
        user.0.changed().await.unwrap();

        let user = match &*user.0.borrow() {
            UserState::Connected(user) => {
                user.clone()
            },
            UserState::Disconnected(err) => {
                panic!("failed to connect to Discord: {}", err)
            },
        };
        
        info!("connected user: {}", user);

        let rp = ActivityBuilder::default()
            .details("Program is started".to_owned())
            .state("Click start to load animation".to_owned())
            .assets(
                Assets::default()
                    .large("dota", Some(""))
            )
            .button(Button {
                label: "Create You Animation by koliy82".to_owned(),
                url: "https://github.com/koliy82/rust_rpc".to_owned(),
            })
            .start_timestamp(started_time);

        info!("updated activity: {:?}",discord.update_activity(rp).await);
        
        Ok(Self {
            app_id,
            subscriptions,
            discord: Arc::new(Mutex::new(discord)),
            user,
            wheel,
            animation: None,
            started_time
        })
    }

    async fn retry_connection(client: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            // Попытка установить новое соединение с Discord
        }
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

        let timestamp_clone = self.started_time;
        
        spawn( async move {
            animation_clone.run(animation_id, timestamp_clone).await;
        });
    }

    pub fn stop_animation(&mut self) {
        if let Some(animation) = &self.animation {
            animation.stop();
            self.animation = None;
        }
    }
}