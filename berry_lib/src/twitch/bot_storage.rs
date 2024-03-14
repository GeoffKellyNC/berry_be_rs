use lazy_static::lazy_static;
use super::twitch_chat::TwitchBot;
use std::sync::Arc;
use dashmap::DashMap;

lazy_static! {
    static ref BOT_STORAGE: DashMap<String, Arc<TwitchBot>> = DashMap::new();
}

pub fn add_bot(channel: String, bot: TwitchBot) {
    BOT_STORAGE.insert(channel, Arc::new(bot));
}

pub fn get_bot(channel: &str) -> Option<Arc<TwitchBot>> {
    BOT_STORAGE.get(channel).map(|r| r.clone())
}

pub fn remove_bot(channel: &str) {
    BOT_STORAGE.remove(channel);
}