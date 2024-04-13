use std::fmt;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing_subscriber::fmt::format;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomAnimation {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image_text: Option<String>,
    pub large_image_key: Option<String>,
    pub small_image_text: Option<String>,
    pub small_image_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomAnimations{
    pub randomize: Option<bool>,
    pub animations: Vec<CustomAnimation>,
}
impl CustomAnimations {
    pub async fn parse(filename: String) -> CustomAnimations{
        let mut file = File::open(format!("{}.json", filename)).await.expect("");
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.expect("");
        let json: CustomAnimations = serde_json::from_str(&contents).expect("JSON was not well-formatted");
        println!("{:?}", json);
        json
    }
    
}