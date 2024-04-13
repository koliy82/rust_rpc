use std::{env, thread};
use std::ops::{Add, Deref};
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
    pub wheel: Arc<Mutex<Wheel>>,
    animation: Option<Animation>,
    pub started_time: SystemTime,
    connect_checker: Option<JoinHandle<()>>,
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
        
        let mut to_ret = Self {
            app_id,
            subscriptions,
            discord: Arc::new(Mutex::new(discord)),
            wheel: Arc::new(Mutex::new(wheel)),
            animation: None,
            started_time,
            connect_checker: None
        };

        to_ret.start_connect_worker();

        Ok(to_ret)
    }

    fn start_connect_worker(&mut self) {
        let wheel_clone = Arc::clone(&self.wheel);
        let connection_worker = tokio::spawn(async move {
            loop {
                let val = wheel_clone.lock().await;
                let mut user_recv = val.user().0;
                if user_recv.changed().await.is_ok() {
                    let user_state = user_recv.borrow_and_update();
                    let user_state = &*user_state;
                    match user_state {
                        UserState::Connected(a) => {
                            info!("User: {a}");
                        }
                        UserState::Disconnected(e) => {
                            info!("Disconnect: {e}");
                        }
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        self.connect_checker = Some(connection_worker);
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