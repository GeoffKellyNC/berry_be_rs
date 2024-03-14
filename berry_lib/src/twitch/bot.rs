// bot.rs
use super::commands::CommandHandler;
use super::twitch_api::{TwitchAPI, TwitchMessage, TwitchError};
use std::io::ErrorKind;


pub struct Bot<'a> {
    api: TwitchAPI<'a>,
    command_handler: CommandHandler,
}

impl<'a> Bot<'a> {
    pub fn new(access_token: &'a str, channel: &'a str) -> Result<Self, TwitchError> {
        let api = TwitchAPI::new(access_token, channel)?;
        let command_handler = CommandHandler::new();
        Ok(Bot { api, command_handler })
    }


    // ...
    
    pub fn run(&mut self) -> Result<(), TwitchError> {
        self.api.connect()?;
        loop {
            match self.api.read_message() {
                Ok(Some(message)) => self.handle_message(&message),
                Ok(None) => {}
                Err(TwitchError::IOError(ref e)) if e.kind() == ErrorKind::WouldBlock => {
                    println!("No messages to read, sleeping..."); // !REMOVE
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn handle_message(&mut self, message: &TwitchMessage) {
        println!("Handling message: {}", message.text); // !REMOVE
        if let Some(command) = self.command_handler.get_command(&message.text) {
            let response = command.execute(&message);
            if let Err(e) = self.api.send_message(&response) {
                eprintln!("Error sending message: {:?}", e);
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), TwitchError> {
        self.api.disconnect()
    }
}