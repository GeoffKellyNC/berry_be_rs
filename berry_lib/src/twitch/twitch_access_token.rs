use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use serde_json;
use std::env;

pub enum TwitchTokenError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
}

impl From<reqwest::Error> for TwitchTokenError {
    fn from(err: reqwest::Error) -> TwitchTokenError {
        TwitchTokenError::RequestError(err)
    }
}

impl From<serde_json::Error> for TwitchTokenError {
    fn from(err: serde_json::Error) -> TwitchTokenError {
        TwitchTokenError::JsonError(err)
    }
}

#[derive(Deserialize, Debug)]
pub struct TwitchAccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

pub async fn get_twitch_access_token(
    code: String,
    client: &Client,
) -> Result<TwitchAccessToken, TwitchTokenError> {
    dotenv().ok();
    println!("Getting Twitch Access Token"); // !REMOVE
    let twitch_code_url = construct_twitch_access_url(&code);

    let res = client.post(&twitch_code_url).send().await?;

    let body = res.text().await;
    let token_data: TwitchAccessToken = match serde_json::from_str(&body.unwrap()) {
        Ok(data) => data,
        Err(e) => return Err(TwitchTokenError::JsonError(e)),
    };

    Ok(token_data)
}

// This Function constructs the URL to request the Twitch Access Token
fn construct_twitch_access_url(code: &str) -> String {
    dotenv().ok(); // Loads environment variables from .env file

    let local_mode = env::var("LOCAL_MODE").unwrap_or_else(|_| "false".to_string());
    let redirect_uri = env::var("TWITCH_REDIRECT_URI").expect("TWITCH_REDIRECT_URI must be set");
    let redirect_uri_local =
        env::var("TWITCH_REDIRECT_URI").expect("TWITCH_REDIRECT_URI_LOCAL must be set");
    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");
    let client_secret = env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET must be set");

    let redirect_uri = if local_mode == "true" {
        &redirect_uri_local
    } else {
        &redirect_uri
    };

    format!(
        "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
        client_id, client_secret, code, redirect_uri
    )
}
