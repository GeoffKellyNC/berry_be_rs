use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use super::twitch_chat::ReceivedMessage;
use colored::*;
use super::bot_storage;

#[derive(Clone)]
pub struct MessageProcessor {
    tx: Sender<ReceivedMessage>,
}

impl MessageProcessor {
    pub fn new() -> (MessageProcessor, Receiver<ReceivedMessage>) {
        let (tx, rx) = channel();

        let message_processor = MessageProcessor { tx };

        (message_processor, rx)
    }

    pub fn send_message(&self, message: ReceivedMessage) {
        self.tx.send(message).unwrap();
    }
}

pub fn start_message_processing(rx: Receiver<ReceivedMessage>) {
    println!("{}", "Starting Messaging Procesor".green());
    thread::spawn(move || {
        for message in rx {
            process_message(message);
        }
    });
}

fn process_message(message: ReceivedMessage) {
    // Perform message processing logic here
    println!("Processing message: {:?}", message);

    if let Some(bot) = bot_storage::get_bot(&message.channel) {
        let response = format!("You said: {}", message.message);
        bot.chat_connection.send_chat_message(&message.channel, &response).unwrap();
    }
}