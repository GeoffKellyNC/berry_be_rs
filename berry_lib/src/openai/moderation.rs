use std::env;
use reqwest;
use serde::{Deserialize, Serialize};
use colored::*;


#[derive(Debug, Deserialize)]
pub struct ModerationResponse {
   pub  id: String,
    pub model: String,
    pub results: Vec<OpenAiModRes>,
}
#[derive(Debug, Deserialize)]
pub struct FlaggedMessage {
    pub text: String,
    pub category: ModerationCategory,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
enum SeverityStatus {
    Minor(FlaggedMessage),
    Moderate(FlaggedMessage),
    High(FlaggedMessage),
    Severe(FlaggedMessage),
}

#[derive(Debug, Deserialize)]
pub enum ModerationCategory {
    Sexual,
    Hate,
    Harassment,
    SelfHarm,
    SexualMinors,
    HateThreatening,
    ViolenceGraphic,
    SelfHarmIntent,
    SelfHarmInstructions,
    HarassmentThreatening,
    Violence,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiModRes {
    pub flagged: bool,
    pub categories: ModerationCategories,
    pub category_scores: ModerationScores,
}



#[derive(Debug, Deserialize)]
pub struct ModerationCategories {
   pub harassment: bool,

    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,

    pub hate: bool,

    #[serde(rename = "hate/threatening")]
    pub  hate_threatening: bool,

    #[serde(rename = "self-harm")]
    pub self_harm: bool,

    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,

    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,

    pub sexual: bool,

    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,

    pubviolence: bool,

    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

#[derive(Debug, Deserialize)]
struct ModerationScores {

    harassment: f64,

    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,

    pub hate: f64,

    #[serde(rename = "hate/threatening")]
    pub  hate_threatening: f64,

    #[serde(rename = "self-harm")]
    pub self_harm: f64,

    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,

    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,

    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,

    pub violence: f64,

    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}

#[derive(Serialize)]
struct ModerationRequest {
    input: String,
}

pub enum ModerationError {
    IoError(std::io::Error),
    ApiError,
    ConnectionError,
}

impl std::fmt::Display for ModerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModerationError::IoError(e) => write!(f, "IO Error: {}", e),
            ModerationError::ApiError => write!(f, "API Error"),
            ModerationError::ConnectionError => write!(f, "Connection Error"),
        }
    }
}

impl From<std::io::Error> for ModerationError {
    fn from(err: std::io::Error) -> Self {
        ModerationError::IoError(err)
    }
}

pub struct OpenAiApiModeration {
    api_key: String,
    input: String,
}

impl OpenAiApiModeration {
    pub fn new(input: &str) -> Self {
        OpenAiApiModeration {
            api_key: env::var("OPEN_AI_KEY").expect("Failed to get Open AI Key"),
            input: input.to_string(),
        }
    }


    pub async fn handle_input_check(&self) -> Result<ModerationResponse, ModerationError> {
        println!("{}", "Handling Input Check".green()); // !REMOVE
        let client = reqwest::Client::new();
        let endpoint = "https://api.openai.com/v1/moderations";
    
        let request_body = ModerationRequest {
            input: self.input.clone(),
        };
    
        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|_| ModerationError::ConnectionError)?;
    
        let moderation_response: ModerationResponse = response
            .json()
            .await
            .map_err(|e| {
                println!("{} {}", "Error Parsing Response".red(), e); // !REMOVE
                ModerationError::ApiError
            })?;
        
        Ok(moderation_response)
    }
}