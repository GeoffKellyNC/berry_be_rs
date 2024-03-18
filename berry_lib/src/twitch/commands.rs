use super::twitch_api::TwitchMessage;
use colored::*;

pub trait Command: Send {
    fn execute(&self, message: &TwitchMessage) -> String;
    fn get_name(&self) -> String;
    fn get_action(&self) -> String;
}

pub struct PingCommand;

impl Command for PingCommand {
    fn execute(&self, _message: &TwitchMessage) -> String {
        "Pong!".to_string()
    }

    fn get_name(&self) -> String {
        "ping".to_string()
    }

    fn get_action(&self) -> String {
        "!ping".to_string()
    }
}

pub struct TestCommand;

impl Command for TestCommand {
    fn execute(&self, _message: &TwitchMessage) -> String {
        "Test Works!".to_string()
    }

    fn get_name(&self) -> String {
        "test".to_string()
    }

    fn get_action(&self) -> String {
        "!test".to_string()
    }
}

pub struct CustomCommand {
    pub name: String,
    pub action: String,
    pub callback: Box<dyn Fn(&TwitchMessage) -> String + Send>,
}

impl Command for CustomCommand {
    fn execute(&self, message: &TwitchMessage) -> String {
        (self.callback)(message)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_action(&self) -> String {
        self.action.clone()
    }
}

pub struct CommandHandler {
    pub builtin_commands: Vec<Box<dyn Command>>,
    pub custom_commands: Vec<CustomCommand>,
}

impl CommandHandler {
    pub fn new<F>(get_custom_commands: F) -> Self
    where
        F: FnOnce() -> Vec<CustomCommand> + Send + 'static,
    {
        let builtin_commands: Vec<Box<dyn Command>> = vec![
            Box::new(PingCommand),
            Box::new(TestCommand),
        ];
        let custom_commands = get_custom_commands();
        CommandHandler {
            builtin_commands,
            custom_commands,
        }
    }

    pub fn get_command(&self, message: &str) -> Option<&dyn Command> {
        println!("Checking for command in message: {}", message.bright_blue().bold()); // !REMOVE

        if message.starts_with('!') {
            let command_name = &message[1..].to_lowercase();
            println!("Command name: {}", command_name.bright_yellow().bold()); // !REMOVE

            for command in &self.builtin_commands {
                println!("Checking builtin command: {}", command.get_action().bright_cyan().bold()); // !REMOVE
                if command.get_name() == command_name.to_string() {
                    println!("Found builtin command: {}", command.get_action().bright_green().bold()); // !REMOVE
                    return Some(command.as_ref());
                }
            }

            for command in &self.custom_commands {
                println!("Checking custom command: {}", command.get_action().bright_magenta().bold()); // !REMOVE
                if command.get_name() == command_name.to_string() {
                    println!("Found custom command: {}", command.get_action().bright_green().bold()); // !REMOVE
                    return Some(command);
                }
            }

            println!("No command found for: {}", command_name.bright_red().bold()); // !REMOVE
        } else {
            println!("Message does not start with '!'"); // !REMOVE
        }

        None
    }
}