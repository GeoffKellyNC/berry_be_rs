use super::twitch_api::TwitchChatAPI;
use colored::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchIdResponse {
    pub data: Vec<TwitchUser>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchUser {
    pub id: String,
    // other fields if needed
}

pub async fn get_user_twitch_id<'a>(
    username: &str,
    api: &'a TwitchChatAPI<'a>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", "Getting User Twitch ID".bright_blue().bold().underline());
    let access_token = api.get_access_token();

    let fmt_access_token = format!("Bearer {}", access_token);


    let base_url = format!("https://api.twitch.tv/helix/users?login={}", username);
    let client_id = std::env::var("TWITCH_CLIENT_ID").expect("Failed to get client id");
    let client = reqwest::Client::new();

    let res = client
        .get(base_url)
        .header("Authorization", fmt_access_token)
        .header("Client-Id", client_id)
        .send()
        .await?
        .text()
        .await?;

    let twitch_id_response: TwitchIdResponse =
        serde_json::from_str(&res).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    if let Some(user) = twitch_id_response.data.first() {
        println!(
            "{} {:?}",
            "USER TWITCH ID:".bright_blue().bold().underline(),
            user.id
        );

        Ok(user.id.clone())
    } else {
        Err("Failed to get user Twitch ID".into())
    }
}
