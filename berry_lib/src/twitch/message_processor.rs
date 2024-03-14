use super::twitch_chat::ReceivedMessage;
use colored::*;
// use super::bot_storage;

#[derive(Clone)]
pub struct MessageProcessor;


pub fn process_message(message: ReceivedMessage) {
    let ReceivedMessage {
        channel,
        username,
        user_id: _,
        message,
    } = message;

    println!(
        "{}{}{}{}{}{}",
        "Channel: ".bright_yellow(),
        channel,
        " | User: ".bright_yellow(),
        username,
        " | Message: ".bright_yellow(),
        message
    );

    if message.starts_with("!") {
        let command = message.split_whitespace().next().unwrap();
        match command {
            "!test" => {
                println!("{}", "Test Command".bright_green());
            }
            _ => {
                println!("{}", "Unknown Command".bright_red());
            }
        }
    }
}