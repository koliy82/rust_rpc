// use std::{env, sync, thread, time};
// use std::sync::Arc;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::thread::JoinHandle;
// use std::time::Duration;
// use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
// use log::{error, info, trace};
// use crate::models::animation::Animation;
// use crate::tray_start;
//
// pub struct Context {
//     client: Arc<DiscordIpcClient>,
//     handle: Option<JoinHandle<()>>,
//     pub alive: Arc<AtomicBool>,
// }
//
// impl Context {
//     pub fn new(client_id: &str) -> Self {
//         Self {
//             client: Arc::new(DiscordIpcClient::new(&client_id).expect("Failed to create client")),
//             handle: None,
//             alive: Arc::new(AtomicBool::new(false)),
//         }
//     }
//
//     pub fn start(&mut self, animation_id: i32, sleep: u64)
//         // where F: 'static + Send + FnMut()
//     {
//         self.alive.store(true, Ordering::SeqCst);
//         let alive = self.alive.clone();
//         let mut client = Arc::clone(&self.client);
//
//         self.handle = Some(thread::spawn(move || {
//             while alive.load(Ordering::SeqCst) {
//                 match client.connect() {
//                     Ok(_) => {
//                         info!("Client connected to discord id {} successfully.", client.client_id.clone());
//                         break;
//                     }
//                     Err(_) => {
//                         error!("Client failed to connect to Discord, next try...");
//                     }
//                 };
//                 Animation::run(animation_id, client);
//                 thread::sleep(Duration::from_secs(sleep));
//             }
//         }));
//     }
//
//     pub fn stop(&mut self) {
//         self.alive.store(false, Ordering::SeqCst);
//         self.handle
//             .take().expect("Called stop on non-running thread")
//             .join().expect("Could not join spawned thread");
//         trace!("Thread closed");
//     }
//
// }