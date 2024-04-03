use crate::openai::moderation::FlaggedMessage;
use colored::Colorize;

// bot.rs
use super::commands::CommandHandler;
use super::commands::CustomCommand;
use super::twitch_api::{TwitchChatAPI, TwitchError, TwitchMessage};
use super::twitch_endpoint;
use crate::openai;
use std::io::ErrorKind;
use std::collections::HashMap;

pub struct Bot<'a> {
    api: TwitchChatAPI<'a>,
    command_handler: CommandHandler,
}

impl<'a> Bot<'a> {
    pub fn new(access_token: &'a str, channel: &'a str) -> Result<Self, TwitchError> {
        let api = TwitchChatAPI::new(access_token, channel)?;

        let command_handler = CommandHandler::new(|| match get_custom_commands() {
            Ok(command) => command,
            Err(e) => {
                println!("Error Getting Commands {e}");
                vec![]
            }
        });
        Ok(Bot {
            api,
            command_handler,
        })
    }

    // ...

    pub async fn run(&mut self) -> Result<(), TwitchError> {
        self.api.connect()?;
        loop {
            match self.api.read_message() {
                Ok(Some(message)) => self.handle_message(&message).await,
                Ok(None) => {}
                Err(TwitchError::IOError(ref e)) if e.kind() == ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn handle_message(&mut self, message: &TwitchMessage) {

        let moderation = openai::moderation::OpenAiApiModeration::new(&message.text);

        match moderation.handle_input_check().await {
            Ok(res) => {

                let is_flagged = res.results[0].flagged;
                let categories = &res.results[0].categories;
                let scores = &res.results[0].category_scores;

                if is_flagged {

                    println!("{}", "=====================================================".bright_yellow().bold());
                    println!("{}", "=====================================================".bright_yellow().bold());

                    println!("{}", "Message is FLAGGED".bright_red().bold());
                    let true_fields = categories.iterate_and_filter_true();

                    println!("{}: {:?}", "Moderation Scores".bright_yellow().bold(), res.results[0].category_scores);

                    println!(
                        "{}: {:?}",
                        "True Fields".bright_yellow().bold(),
                        true_fields
                    );


                    let offence = match determine_offence(true_fields) {
                        Some(offence) => offence,
                        None => {
                            "No Offence Found".to_string();
                            return
                        },
                    };


                    let offender_name = &message.sender;
                    let score = scores.get_score(&offence);
                    let user_text = &message.text;

                    println!(
                        "{} {}: {} {} {} {}",
                        "OFFENCE".red().bold().underline(),
                        offender_name,
                        offence,
                        score,
                        "USER TEXT".bright_purple().bold().underline(),
                        user_text
                    );

                    let offender_twitch_id = match twitch_endpoint::get_user_twitch_id(
                        &offender_name,
                        &self.api,
                    )
                    .await
                    {
                        Ok(id) => id,
                        Err(e) => {
                            println!(
                                "{} {e}",
                                "ERROR GETTING TWITCH TOKEN: ".bright_red().bold().underline()
                            );
                            return;
                        }
                    };
                    
                    let flagged_message = FlaggedMessage::new(
                        &offender_name,
                        &offender_twitch_id,
                        &user_text,
                        &offence,
                        score,
                    );

                    moderation.moderate_input(flagged_message);

                    return;
                }
                if let Some(command) = self.command_handler.get_command(&message.text) {
             
                    let response = command.execute(&message);
                    if let Err(e) = self.api.send_message(&response) {
                        eprintln!("Error sending message: {:?}", e);
                    }
                } else {
                    ()
                }
            }
            Err(e) => {
                eprintln!("Error Handling Moderation: {e}");
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), TwitchError> {
        self.api.disconnect()
    }
}

fn get_custom_commands() -> Result<Vec<CustomCommand>, Box<dyn std::error::Error>> {
    Ok(vec![CustomCommand {
        name: "hello".to_string(),
        action: "!hello".to_string(),
        callback: Box::new(|message| format!("Hello from Rust! {}", message.text)),
    }])
}


fn determine_offence(categories: Vec<String>) -> Option<String> {
    let severities: HashMap<&str, i32> = HashMap::from([
        ("sexual_minors", 1),
        ("hate", 2),
        ("self_harm", 3),
        ("self_harm_intent", 4),
        ("hate_threatening", 5),
        ("self_harm_instructions", 6),
        ("harassment_threatening", 7),
        ("sexual", 8),
        ("violence_graphic", 9),
        ("violence", 10),
        ("harassment", 11),
    ]);

    let cat = categories
        .into_iter()
        .filter_map(|category| severities.get(&category.as_str()).copied())
        .min()
        .map(|severity| {
            severities
                .iter()
                .find(|&(_, s)| *s == severity)
                .map(|(category, _)| category.to_string())
                .unwrap_or_else(|| "No Offence Found".to_string())
        })
        .unwrap_or_else(|| "No Offence Found".to_string());

    Some(cat)
}