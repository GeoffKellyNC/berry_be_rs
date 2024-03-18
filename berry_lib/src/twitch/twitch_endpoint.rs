use reqwest;
use super::twitch_api::TwitchChatAPI;
use serde::{Deserialize, Serialize};
use colored::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchId {
    id: String,
}


pub async fn get_user_twitch_id<'a>(
    username: &str, 
    api: &'a TwitchChatAPI<'a>
) -> Result<TwitchId, Box<dyn std::error::Error>> {

    let access_token = api.get_access_token();
    let base_url = format!("https://api.twitch.tv/helix/users?login={}", username);
    let client_id = std::env::var("TWITCH_CLIENT_ID").expect("Failed to get client id");
    let client = reqwest::Client::new();


    let res =client
        .get(base_url)
        .header("Authorization", access_token)
        .header("Client-Id", client_id)
        .send()
        .await?
        .text()
        .await?;

    let twitch_id: TwitchId = 
        serde_json::from_str(&res).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    println!("{} {:?}", "USER TWITCH ID:".bright_blue().bold().underline(), twitch_id);

    Ok(twitch_id)




} 