use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchResponse {
    pub data: Vec<TwitchUserData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitchUserData {
    #[serde(rename = "id")]
    pub twitch_id: String,

    #[serde(rename = "login")]
    pub twitch_login: String,

    #[serde(rename = "description")]
    pub twitch_description: Option<String>,

    #[serde(rename = "profile_image_url")]
    pub twitch_image: Option<String>,

    #[serde(rename = "email")]
    pub twitch_email: Option<String>, // Note: Email might not always be present.

    #[serde(rename = "broadcaster_type")]
    pub broadcast_type: Option<String>,

    #[serde(rename = "view_count", deserialize_with = "string_or_number")]
    pub view_count: Option<i32>, // Adjusting to i32, assuming view_count is numeric.

    #[serde(rename = "created_at")]
    pub twitch_created: Option<String>,
}

fn string_or_number<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
        Err(_) => Ok(None), // If it's not a string, handle accordingly.
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTwitchData {
    pub unxid: String,
    pub twitch_id: String,
    pub twitch_login: String,
    pub twitch_description: Option<String>,
    pub twitch_image: Option<String>,
    pub twitch_email: String,
    pub broadcast_type: Option<String>,
    pub view_count: Option<String>,
    pub twitch_created: Option<String>,
    pub app_created: String,
}

// For Debugging
impl std::fmt::Display for UserTwitchData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UserTwitchData: unxid: {}, twitch_id: {}, twitch_login: {}, twitch_email: {}, app_created: {}", self.unxid, self.twitch_id, self.twitch_login, self.twitch_email, self.app_created)
    }
}

pub async fn get_user_from_twitch(
    token: &str,
    client: &Client,
) -> Result<Vec<TwitchUserData>, Box<dyn std::error::Error>> {
    let base_url = String::from("https://api.twitch.tv/helix/users");
    let auth_code = format!("Bearer {}", token);
    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");

    let res = client
        .get(&base_url)
        .header("Authorization", auth_code)
        .header("Client-Id", client_id)
        .send()
        .await?
        .text()
        .await?;

    let user_data: TwitchResponse =
        serde_json::from_str(&res).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(user_data.data)
}
