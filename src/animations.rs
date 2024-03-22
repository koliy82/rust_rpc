use std::fmt::format;
use std::thread;
use std::time::Duration;
use discord_rich_presence::activity::Activity;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use log::{info, trace};
use ringbuffer::{AllocRingBuffer, RingBuffer};

pub enum Animations{
    DeadInside,
    Loading
}

impl From<i32> for Animations {
    fn from(id: i32) -> Self {
        match id {
            1 => Animations::DeadInside,
            2 => Animations::Loading,
            _ => Animations::DeadInside,
        }
    }
    
}

impl Animations {
    pub fn run(animation_id: i32, client: &mut DiscordIpcClient) {
        match Animations::from(animation_id) {
            Animations::DeadInside => dead_inside(client),
            Animations::Loading => loading(client),
        }
    }
}

fn dead_inside(client: &mut DiscordIpcClient) {
    info!("Starting Dead Inside 1000-7 Animation");
    let mut i = 1000;
    while i > 0 {
        let formatted_str =  format!("{} - 7 = {}", i, i-7);
        let payload = Activity::new()
            .state("▁ ▂ ▃ ▄ ▅ ▆ ▇ █ ▇ ▆ ▅ ▄ ▃ ▁")
            .details(formatted_str.as_str());
        client.set_activity(payload).expect("failed to set client activity");
        thread::sleep(Duration::from_secs(5));
        trace!("{} - 7 = {}", i, i - 7);
        i -= 7;
    }
}

fn loading(client: &mut DiscordIpcClient){
    info!("Starting Loading Animation");
    let  buffer: AllocRingBuffer<&str> = AllocRingBuffer::from(vec!["▁","▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▁"]);
    let iter = buffer.into_iter(); 
    for element in iter {
        let formatted_str = format!("{} load", element.clone());
        let payload = Activity::new()
            .state(&formatted_str)
            .details("Loading...");
        client.set_activity(payload).expect("failed to set client activity");
        trace!("{}", element);
        thread::sleep(Duration::from_secs(5));
    }
}