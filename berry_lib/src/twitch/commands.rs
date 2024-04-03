use super::twitch_api::TwitchMessage;

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
        let builtin_commands: Vec<Box<dyn Command>> =
            vec![Box::new(PingCommand), Box::new(TestCommand)];
        let custom_commands = get_custom_commands();
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

        } else {
            ()
        }

        None
    }
}
