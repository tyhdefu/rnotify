use serde::{Serialize, Deserialize};
use crate::{Level, Message};
use crate::message::component::Component;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationConfigEntry<T> {
    component: Option<Component>,
    #[serde(default = "Level::min")]
    min_level: Level,
    #[serde(default = "Level::max")]
    max_level: Level,
    notify: T,
}

impl<T> NotificationConfigEntry<T> {
    pub fn matches(&self, m: &Message) -> bool {
        &self.min_level <= m.get_level() && m.get_level() <= &self.max_level
    }

    pub fn get_notify(&self) -> &T {
        &self.notify
    }
}