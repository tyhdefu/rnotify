use serde::{Serialize, Deserialize};
use crate::{Level, Message};
use crate::message::component::Component;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageNotifyConditionConfigEntry<T> {
    #[serde(flatten)]
    message_condition: MessageCondition,
    notify: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MessageCondition {
    component: Option<Component>,
    #[serde(default = "Level::min")]
    min_level: Level,
    #[serde(default = "Level::max")]
    max_level: Level,
}

impl MessageCondition {
    pub fn matches(&self, m: &Message) -> bool {
        if let Some(c) = &self.component {
            if m.get_component().is_none() || !m.get_component().as_ref().unwrap().is_child_of(c) {
                return false;
            }
        }
        &self.min_level <= m.get_level() && m.get_level() <= &self.max_level
    }
}

impl<T> MessageNotifyConditionConfigEntry<T> {
    pub fn matches(&self, m: &Message) -> bool {
        self.message_condition.matches(m)
    }

    pub fn get_notify(&self) -> &T {
        &self.notify
    }
}