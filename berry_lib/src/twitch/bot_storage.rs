use std::collections::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use super::twitch_chat::TwitchBot;

lazy_static! {
    static ref BOT_STORAGE: RwLock<HashMap<String, TwitchBot>> = RwLock::new(HashMap::new());
}

pub fn add_bot(channel: String, bot: TwitchBot) {
    BOT_STORAGE.write().insert(channel, bot);
}

pub fn get_bot(channel: &str) -> Option<TwitchBot> {
    BOT_STORAGE.read().get(channel).cloned()
}

pub fn remove_bot(channel: &str) {
    BOT_STORAGE.write().remove(channel);
}