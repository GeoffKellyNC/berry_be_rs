use colored::Colorize;
use crate::openai::moderation::FlaggedMessage;

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

        println!("Handling message: {}", &message.text); // !REMOVE

        let moderation = openai::moderation::OpenAiApiModeration::new(&message.text);

        match moderation.handle_input_check().await {
            Ok(res) => {


                let is_flagged = res.results[0].flagged;
                let categories = &res.results[0].categories;
                let scores = &res.results[0].category_scores;
                

                if is_flagged {


                    println!("{}", "Message is FLAGGED".bright_red().bold());
                    let true_fields = categories.iterate_and_filter_true();

                    println!("{}: {:?}", "True Fields".bright_yellow().bold(), true_fields);
                    
                    // get first element in true_fields vector
                    let offence = match true_fields.first() {
                        Some(offence) => offence,
                        None => "No Offence Found",
                    };

                    let offender_name = &message.sender;
                    let score = scores.get_score(&offence);
                    let user_text = &message.text;


                    println!("{} {}: {} {} {} {}", "OFFENCE".red().bold().underline(), offender_name, offence, score, "USER TEXT".bright_purple().bold().underline(), user_text);


                    let flagged_message = FlaggedMessage::new(&offender_name, "1234TEST", &user_text, offence, score);

                    moderation.moderate_input(flagged_message);

                    return
                }

                println!("{}", "Message Passed Moderation".bright_green().bold().underline());

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