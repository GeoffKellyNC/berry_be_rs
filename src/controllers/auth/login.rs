//##############################################
// LOGIN ROUTE
// Endpoint: /auth/login
// Method: POST
// Request Body: code (String)
//##############################################

use crate::models::user::set_user_db::{set_user_to_db, SetUserReturn};
use actix_web::http::StatusCode;
use actix_web::web;
use berry_lib::api::api_response::ApiResponse;
use berry_lib::auth::jwt;
use berry_lib::twitch::twitch_user_data::{TwitchUserData, UserTwitchData};
use berry_lib::twitch::{
    self,
    twitch_access_token::{self, TwitchAccessToken, TwitchTokenError},
};
use colored::*;
use reqwest::Client;
use sqlx::PgPool;
use berry_lib::twitch::bot::Bot;

#[derive(serde::Deserialize)]
pub struct LoginResponse {
    code: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]

pub struct LoginApiRes {
    token: String,
    data: UserTwitchData,
}

// ** MAIN LOGIN FUNCTION START **

pub async fn login_twitch(
    pool: web::Data<PgPool>,
    data: web::Json<LoginResponse>,
    reqwest_client: web::Data<Client>,
) -> ApiResponse<LoginApiRes> {
    
    let res_data = data.into_inner();

    let code = res_data.code;


    if code.is_empty() {
        return ApiResponse::new(
            None,
            Some("Code is required".to_string()),
            Some(StatusCode::BAD_REQUEST),
        );
    }

    let twitch_creds = match retrieve_twitch_creds(code, &reqwest_client).await {
        Ok(data) => data,
        Err(e) => match e {
            TwitchTokenError::JsonError(err) => {
                eprintln!("{} {}", "Error Getting Twitch Creds...".red(), err);
                return ApiResponse::new(
                    None,
                    Some(err.to_string()),
                    Some(StatusCode::INTERNAL_SERVER_ERROR),
                );
            }
            TwitchTokenError::RequestError(err) => {
                eprintln!("{} {}", "Error Getting Twitch Creds...".red(), err);
                return ApiResponse::new(
                    None,
                    Some(err.to_string()),
                    Some(StatusCode::INTERNAL_SERVER_ERROR),
                );
            }
        },
    };

    let twitch_data =
        match retrieve_twitch_data(twitch_creds.access_token.clone(), &reqwest_client).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to get twitch data: {} ", e);
                return ApiResponse::new(
                    None,
                    Some(e.to_string()),
                    Some(StatusCode::INTERNAL_SERVER_ERROR),
                );
            }
        };

    let twitch_data = twitch_data[0].clone();

    let user_data = match get_and_set_user_to_db(twitch_data, &pool).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error Setting user to DB {}", e);
            return ApiResponse::new(
                None,
                Some("Error Setting User to db".to_string()),
                Some(StatusCode::NOT_FOUND),
            );
        }
    };

    let channel_to_join = user_data.twitch_login.clone();

    // FOR TESTING

    // let channel_to_join = String::from("");

    initiate_twitch_bot(twitch_creds.access_token.clone(), channel_to_join);


    let jwt_token = match init_and_get_jwt(twitch_creds.access_token, &user_data).await {
        Ok(token) => token,
        Err(e) => return ApiResponse::new(None, Some(e), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };


    let full_response = LoginApiRes {
        token: jwt_token,
        data: user_data,
    };


    ApiResponse::new(Some(full_response), None, Some(StatusCode::OK))
}

// ** MAIN LOGIN FUNCTION END **

fn initiate_twitch_bot(token: String, channel: String) {
    tokio::spawn(async move {
        match Bot::new(&token, &channel) {
            Ok(mut bot) => {
                println!("Bot Created!"); // !REMOVE
                if let Err(e) = bot.run().await {
                    eprintln!("Error running bot: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Error creating bot: {:?}", e);
            }
        }
    });
}

// Retrieve Twitch Creds

async fn retrieve_twitch_creds(
    code: String,
    reqwest_client: &Client,
) -> Result<TwitchAccessToken, TwitchTokenError> {
    let twitch_creds = twitch_access_token::get_twitch_access_token(code, &reqwest_client).await;

    match twitch_creds {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

// Retrieve Twitch Data
async fn retrieve_twitch_data(
    token: String,
    reqwest_client: &Client,
) -> Result<Vec<TwitchUserData>, Box<dyn std::error::Error>> {
    let twitch_data_raw =
        twitch::twitch_user_data::get_user_from_twitch(&token, &reqwest_client).await;

    match twitch_data_raw {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn get_and_set_user_to_db(
    data: TwitchUserData,
    pool: &PgPool,
) -> Result<UserTwitchData, sqlx::Error> {
    let user_data = set_user_to_db(&data, &pool).await;

    match user_data {
        Ok(data) => match data {
            SetUserReturn::UserExists(user) => Ok(user),
            SetUserReturn::NotFound(user) => Ok(user),
        },
        Err(e) => Err(e),
    }
}

async fn init_and_get_jwt(token: String, user_data: &UserTwitchData) -> Result<String, String> {
    let jwt_algo = jwt::JwtAlgorithm::RS512;
    let jwt_config = jwt::JwtConfig::new(jwt_algo.into());
    let jwt_exp = jwt::JwtConfig::generate_exp_time(2);

    let claims = jwt::Claims {
        unxid: user_data.unxid.to_string(),
        exp: jwt_exp,
        twitch_access_token: token,
        twitch_id: user_data.twitch_id.to_string(),
    };

    let jwt_token = match jwt_config.generate_token(&claims) {
        Ok(token) => token,
        Err(_) => return Err("Failed to generate JWT Token".to_string()),
    };

    Ok(jwt_token)
}
