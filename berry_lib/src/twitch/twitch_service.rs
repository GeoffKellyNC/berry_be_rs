use std::collections::HashMap;
use super::twitch_api::{TwitchMessage, TwitchError};
use super::bot::Bot;
use crate::openai;

pub struct TwitchService<'a> {
    users: HashMap<String, String>,
    bot: Option<Bot<'a>>,
    bot_token: String,
    bot_channel: String,
}

impl<'a> TwitchService<'a> {
    pub fn new(bot_token: String, bot_channel: String) -> Self {
        TwitchService {
            users: HashMap::new(),
            bot: None,
            bot_token,
            bot_channel,
        }
    }

    pub async fn run(&mut self) {
        // Initialize the bot and connect to Twitch IRC
        let bot = Bot::new(&self.bot_token, &self.bot_channel).unwrap();
        self.bot = Some(bot);

        loop {
            // Process incoming messages and handle user actions
            if let Some(bot) = &mut self.bot {
                match bot.api.read_message() {
                    Ok(Some(message)) => self.handle_message(&message).await,
                    Ok(None) => {}
                    Err(TwitchError::IOError(ref e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                    Err(e) => {
                        eprintln!("Error reading message: {:?}", e);
                    }
                }
            }
        }
    }

    async fn handle_message(&mut self, message: &TwitchMessage) {
        println!("Handling message: {}", &message.text);

        let moderation = openai::moderation::OpenAiApiModeration::new(&message.text);

        match moderation.handle_input_check().await {
            Ok(res) => {
                println!("Moderation Response: {:?}", res);

                if let Some(bot) = &mut self.bot {
                    if let Some(command) = bot.command_handler.get_command(&message.text) {
                        let response = command.execute(&message);
                        if let Err(e) = bot.api.send_message(&response) {
                            eprintln!("Error sending message: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error Handling Moderation: {e}");
            }
        }
    }

    pub fn connect_user(&mut self, token: String, channel: String) {
        self.users.insert(channel, token);
        // Perform any necessary actions when a user connects
        // ...
    }
}