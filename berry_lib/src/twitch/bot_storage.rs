use std::collections::HashMap;
use lazy_static::lazy_static;

use super::twitch_chat::TwitchBot;

lazy_static! {
    static ref BOT_STORAGE: HashMap<String, TwitchBot> = HashMap::new();
}

pub fn add_bot(channel: String, bot: TwitchBot) {
    BOT_STORAGE.insert(channel, bot);
}

pub fn get_bot(channel: &str) -> Option<&TwitchBot> {
   let bot = BOT_STORAGE.get(channel);

    match bot {
         Some(bot) => Some(bot),
         None => None,
    }
}

pub fn remove_bot(channel: &str) {
    let _ = BOT_STORAGE.remove(channel);
}
