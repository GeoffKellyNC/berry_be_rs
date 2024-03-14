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

pub struct CustomCommand {
    pub name: String,
    pub response: String,
}

impl Command for CustomCommand {
    fn execute(&self, _message: &TwitchMessage) -> String {
        self.response.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct CommandHandler {
    pub builtin_commands: Vec<Box<dyn Command>>,
    pub custom_commands: Vec<CustomCommand>,
}

impl CommandHandler {
    pub fn new(custom_commands: Vec<CustomCommand>) -> Self {
        let builtin_commands: Vec<Box<dyn Command>> = vec![
            Box::new(PingCommand),
            Box::new(TestCommand),
        ];
        CommandHandler {
            builtin_commands,
            custom_commands,
        }
    }

    pub fn get_command(&self, message: &str) -> Option<&dyn Command> {
        if message.starts_with('!') {
            let command_name = &message[1..].to_lowercase();
            for command in &self.builtin_commands {
                if command.get_name() == command_name.to_string() {
                    return Some(command.as_ref());
                }
            }
            for command in &self.custom_commands {
                if command.get_name() == command_name.to_string() {
                    return Some(command);
                }
            }
        }
        None
    }
}