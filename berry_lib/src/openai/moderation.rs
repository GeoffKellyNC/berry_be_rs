use colored::*;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

enum PunishmentAction {
    Timeout(u64),
    Ban,
    Delete,
    Warn,
    None,
}

struct DefaultThresholds {
    harassment: f64,
    harassment_threatening: f64,
    hate: f64,
    hate_threatening: f64,
    self_harm: f64,
    self_harm_instructions: f64,
    self_harm_intent: f64,
    sexual: f64,
    sexual_minors: f64,
    violence: f64,
    violence_graphic: f64,
}

#[derive(Debug, Deserialize)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<OpenAiModRes>,
}

#[derive(Debug, Deserialize)]
pub struct FlaggedMessage {
    pub username: String,
    pub user_id: String,
    pub text: String,
    pub category: String,
    pub score: f64,
}

impl FlaggedMessage {
    pub fn new(username: &str, user_id: &str, text: &str, category: &str, score: f64) -> Self {
        FlaggedMessage {
            username: String::from(username),
            user_id: String::from(user_id),
            text: String::from(text),
            category: String::from(category),
            score,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAiModRes {
    pub flagged: bool,
    pub categories: ModerationCategories,
    pub category_scores: ModerationScores,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModerationCategories {
    pub harassment: bool,

    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,

    pub hate: bool,

    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,

    #[serde(rename = "self-harm")]
    pub self_harm: bool,

    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,

    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,

    pub sexual: bool,

    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,

    pub violence: bool,

    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

impl ModerationCategories {
    pub fn iterate_and_filter_true(&self) -> Vec<String> {
        let serialized = serde_json::to_value(self).unwrap();
        let mut true_fields = Vec::new();

        if let Value::Object(map) = serialized {
            for (key, value) in map {
                if let Value::Bool(true) = value {
                    true_fields.push(key);
                }
            }
        }

        true_fields
    }
}

#[derive(Debug, Deserialize)]
pub struct ModerationScores {
    pub harassment: f64,

    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,

    pub hate: f64,

    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,

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

impl ModerationScores {
    pub fn get_score(&self, field: &str) -> f64 {
        match field {
            "harassment" => self.harassment,
            "harassment/threatening" => self.harassment_threatening,
            "hate" => self.hate,
            "hate/threatening" => self.hate_threatening,
            "self-harm" => self.self_harm,
            "self-harm/instructions" => self.self_harm_instructions,
            "self-harm/intent" => self.self_harm_intent,
            "sexual" => self.sexual,
            "sexual/minors" => self.sexual_minors,
            "violence" => self.violence,
            "violence/graphic" => self.violence_graphic,
            _ => 0.0,
        }
    }
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

        let moderation_response: ModerationResponse = response.json().await.map_err(|e| {
            println!("{} {}", "Error Parsing Response".red(), e); // !REMOVE
            ModerationError::ApiError
        })?;

        Ok(moderation_response)
    }

    pub fn moderate_input(&self, mod_results: FlaggedMessage) {
        let default_thresholds = DefaultThresholds {
            harassment: 0.950,
            harassment_threatening: 0.970,
            hate: 0.55,
            hate_threatening: 0.960,
            self_harm: 0.980,
            self_harm_instructions: 0.970,
            self_harm_intent: 0.950,
            sexual: 0.88,
            sexual_minors: 0.50,
            violence: 0.95,
            violence_graphic: 0.990,
        };

        println!("{}", "Moderating Input".bright_red().bold().underline()); // !REMOVE
        println!(
            "{}",
            "Moderation Results: ".bright_purple().bold().underline()
        ); // !REMOVE

        let (threshold, punishment) = match &mod_results.category[..] {
            "harassment" => (default_thresholds.harassment, PunishmentAction::Warn),
            "harassment/threatening" => (
                default_thresholds.harassment_threatening,
                PunishmentAction::Ban,
            ),
            "hate" => (default_thresholds.hate, PunishmentAction::Timeout(60)),
            "hate/threatening" => (default_thresholds.hate_threatening, PunishmentAction::Ban),
            "self-harm" => (default_thresholds.self_harm, PunishmentAction::Delete),
            "self-harm/instructions" => (
                default_thresholds.self_harm_instructions,
                PunishmentAction::Delete,
            ),
            "self-harm/intent" => (
                default_thresholds.self_harm_intent,
                PunishmentAction::Timeout(120),
            ),
            "sexual" => (default_thresholds.sexual, PunishmentAction::Delete),
            "sexual/minors" => (default_thresholds.sexual_minors, PunishmentAction::Ban),
            "violence" => (default_thresholds.violence, PunishmentAction::Timeout(30)),
            "violence/graphic" => (
                default_thresholds.violence_graphic,
                PunishmentAction::Delete,
            ),
            _ => (0.0, PunishmentAction::None),
        };

        let rounded_score = round_to_decimal_places(mod_results.score);

        if rounded_score >= threshold {
            println!(
                "{}: {} {} {}",
                mod_results.category.bright_yellow().bold(),
                "Flagged".bright_red().bold(),
                mod_results.username,
                mod_results.user_id
            );
            println!("{}: {}", "Score".bright_yellow().bold(), rounded_score);

            match punishment {
                PunishmentAction::Timeout(duration) => {
                    println!(
                        "{}: {} {} {}",
                        "Punishment Issues".bright_cyan().bold().underline(),
                        mod_results.username,
                        mod_results.text.on_bright_green(),
                        mod_results.category.red()
                    );

                    println!("{}: {} seconds", "Timeout".bright_cyan().bold(), duration);
                }
                PunishmentAction::Ban => {
                    println!(
                        "{}: {} {} {}",
                        "Punishment Issues".bright_cyan().bold().underline(),
                        mod_results.username,
                        mod_results.text,
                        mod_results.category
                    );

                    println!(
                        "{}: {}",
                        "Punishment".bright_cyan().bold(),
                        "Ban".bright_red().bold()
                    );
                }
                PunishmentAction::Delete => {
                    println!(
                        "{}: {} {} {}",
                        "Punishment Issues".bright_cyan().bold().underline(),
                        mod_results.username,
                        mod_results.text,
                        mod_results.category
                    );

                    println!(
                        "{}: {}",
                        "Punishment".bright_cyan().bold(),
                        "Delete".bright_red().bold()
                    );
                }
                PunishmentAction::Warn => {
                    println!(
                        "{}: {} {} {}",
                        "Punishment Issues".bright_cyan().bold().underline(),
                        mod_results.username,
                        mod_results.text,
                        mod_results.category
                    );

                    println!(
                        "{}: {}",
                        "Punishment".bright_cyan().bold(),
                        "Warn".bright_yellow().bold()
                    );
                }
                PunishmentAction::None => {
                    println!(
                        "{}: {} {} {}",
                        "Punishment Issues".bright_cyan().bold().underline(),
                        mod_results.username,
                        mod_results.text,
                        mod_results.category
                    );

                    println!(
                        "{}: {}",
                        "Punishment".bright_cyan().bold(),
                        "None".bright_green().bold()
                    );
                }
            }
        } else {
            println!(
                "{}: {}",
                "No Offence Found".bright_yellow().bold(),
                "Not Flagged".bright_green().bold()
            );

            println!("{}: {}", "Score".bright_yellow().bold(), rounded_score);

            println!("{}", "=====================================================".bright_yellow().bold());
            println!("{}", "=====================================================".bright_yellow().bold());
        }
    }
}

fn round_to_decimal_places(value: f64) -> f64 {
    let multiplier = 10_u64.pow(3) as f64;
    (value * multiplier).round() / multiplier
}
