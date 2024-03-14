// commands.rs
use super::twitch_api::TwitchMessage;

pub trait Command {
    fn execute(&self, message: &TwitchMessage) -> String;
    fn get_name(&self) -> String;
}

pub struct PingCommand;

impl Command for PingCommand {
    fn execute(&self, _message: &TwitchMessage) -> String {
        "Pong!".to_string()
    }

    fn get_name(&self) -> String {
        "ping".to_string()
    }
}

pub struct TestCommand;

impl Command for TestCommand {
    fn execute(&self, _message: &TwitchMessage) -> String {
        "Hello from Rust!".to_string()
    }

    fn get_name(&self) -> String {
        "test".to_string()
    }
}

pub struct CommandHandler {
    commands: Vec<Box<dyn Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(PingCommand),
            Box::new(TestCommand),
        ];
        CommandHandler { commands }
    }

    pub fn get_command(&self, message: &str) -> Option<&dyn Command>{
        if message.starts_with('!') {
            let command_name = &message[1..];
            for command in &self.commands {
                if command.get_name().to_lowercase() == command_name {
                    println!("Found command: {}", command.get_name()); // !REMOVE
                    return Some(command.as_ref());
                }
            }
        }
        None
    }
}