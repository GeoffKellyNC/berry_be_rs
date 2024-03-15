// bot.rs
use super::commands::CommandHandler;
use super::twitch_api::{TwitchAPI, TwitchMessage, TwitchError};
use std::io::ErrorKind;
use super::commands::CustomCommand;
use crate::openai;


pub struct Bot<'a> {
    api: TwitchAPI<'a>,
    command_handler: CommandHandler,
}

impl<'a> Bot<'a> {
    pub fn new(access_token: &'a str, channel: &'a str) -> Result<Self, TwitchError> {
        let api = TwitchAPI::new(access_token, channel)?;
        let custom_commands: Vec<CustomCommand> = vec![
            CustomCommand {
                name: "hello".to_string(),
                response: "Hello, world!".to_string(),
            },
        ]; // TODO: This will be an SQL query that will get users custom commands.
        let command_handler = CommandHandler::new(custom_commands);
        Ok(Bot { api, command_handler })
    }

    
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

        println!("Handling message: {}", &message.text); // !REMOVE

        let moderation = openai::moderation::OpenAiApiModeration::new(&message.text);

        match moderation.handle_input_check().await {
            Ok(res) => {
                
                println!("Moderation Response: {:?}", res); // !REMOVE

                if let Some(command) = self.command_handler.get_command(&message.text) {

                    let response = command.execute(&message);
        
                    if let Err(e) = self.api.send_message(&response) {
                        
                        eprintln!("Error sending message: {:?}", e);
                    }
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