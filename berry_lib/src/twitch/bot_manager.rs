use std::collections::HashMap;
use super::bot::Bot;

pub struct BotManager {
    bots: HashMap<String, tokio::task::JoinHandle<()>>,
}

impl BotManager {
    pub fn new() -> Self {
        BotManager {
            bots: HashMap::new(),
        }
    }

    pub async fn connect_bot(&mut self, token: String, channel: String) {
        if !self.bots.contains_key(&channel) {
            let channel_clone = channel.clone();
            let bot_task = tokio::spawn(async move {
                match Bot::new(&token, &channel_clone) {
                    Ok(mut bot) => {
                        if let Err(e) = bot.run().await {
                            eprintln!("Error running bot for {}: {:?}", channel_clone, e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error creating bot for {}: {:?}", channel_clone, e);
                    }
                }
            });
            self.bots.insert(channel, bot_task);
        }
    }
}