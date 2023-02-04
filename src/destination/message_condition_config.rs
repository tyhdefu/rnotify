use serde::{Serialize, Deserialize};
use crate::message::component::Component;
use crate::message::{Level, Message};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageNotifyConditionConfigEntry<T> {
    #[serde(flatten)]
    message_condition: MessageCondition,
    notify: T,
}

/// A filter for a [`Message`]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MessageCondition {
    /// If present, then Messages must be a child of (or same as) this component as per [Component::is_child_of]
    /// ```rust
    /// use rnotifylib::destination::message_condition_config::MessageCondition;
    /// use rnotifylib::message::builder::MessageBuilder;
    /// use rnotifylib::message::component::Component;
    /// use rnotifylib::message::Message;
    ///
    /// let condition_component = Component::from("database/backup");
    /// let condition = MessageCondition::of_component(condition_component);
    ///
    /// fn make_message(component: &str) -> Message {
    ///     let mut message_builder = MessageBuilder::new();
    ///     message_builder.component(Component::from(component));
    ///     message_builder.build()
    /// }
    ///
    /// assert!(condition.matches(&make_message("database/backup")), "Should match itself");
    ///
    /// assert!(condition.matches(&make_message("database/backup/table1")), "Should match child");
    /// assert!(condition.matches(&make_message("database/backup/table2")), "Should match child");
    ///
    /// assert!(!condition.matches(&make_message("database/uptime")), "Should not match - not to do with database backup");
    /// assert!(!condition.matches(&make_message("fish_and_chip_shop/fries")), "Should not match - not to do with database");
    /// ```
    component: Option<Component>,
    /// Messages with a [`Level`] below this will not match this filter
    /// ```rust
    /// use rnotifylib::destination::message_condition_config::MessageCondition;
    /// use rnotifylib::message::builder::MessageBuilder;
    /// use rnotifylib::message::Level;
    ///
    /// let condition = MessageCondition::of_min(Level::Warn);
    ///
    /// let mut message_builder = MessageBuilder::new();
    ///
    /// message_builder.level(Level::Info);
    /// let message = message_builder.build_clone();
    /// assert!(!condition.matches(&message), "Info < Warn, so should not let through");
    ///
    /// message_builder.level(Level::Warn);
    /// let message = message_builder.build_clone();
    /// assert!(condition.matches(&message), "Warn >= Warn, so should be let through");
    ///
    /// message_builder.level(Level::Error);
    /// let message = message_builder.build_clone();
    /// assert!(condition.matches(&message), "Error >= Warn, so should let through")
    ///
    /// ```
    #[serde(default = "Level::min")]
    min_level: Level,
    /// Messages with a [`Level`] above this will not match this filter
    /// ```rust
    /// use rnotifylib::destination::message_condition_config::MessageCondition;
    /// use rnotifylib::message::builder::MessageBuilder;
    /// use rnotifylib::message::Level;
    ///
    /// let condition = MessageCondition::of_max(Level::Warn);
    ///
    /// let mut message_builder = MessageBuilder::new();
    ///
    /// message_builder.level(Level::Info);
    /// let message = message_builder.build_clone();
    /// assert!(condition.matches(&message), "Info <= Warn, so should let through");
    ///
    /// message_builder.level(Level::Warn);
    /// let message = message_builder.build_clone();
    /// assert!(condition.matches(&message), "Warn <= Warn, so should be let through");
    ///
    /// message_builder.level(Level::Error);
    /// let message = message_builder.build_clone();
    /// assert!(!condition.matches(&message), "Error > Warn,  so should not let through")
    ///
    /// ```
    #[serde(default = "Level::max")]
    max_level: Level,
}

impl MessageCondition {
    pub fn new(component: Option<Component>, min_level: Level, max_level: Level) -> Self {
        Self {
            component,
            min_level,
            max_level,
        }
    }

    pub fn of_component(component: Component) -> Self {
        Self {
            component: Some(component),
            ..Default::default()
        }
    }

    pub fn of_min(min_level: Level) -> Self {
        Self {
            min_level,
            ..Default::default()
        }
    }

    pub fn of_max(max_level: Level) -> Self {
        Self {
            max_level,
            ..Default::default()
        }
    }

    pub fn matches(&self, m: &Message) -> bool {
        if let Some(c) = &self.component {
            if m.get_component().is_none() || !m.get_component().as_ref().unwrap().is_child_of(c) {
                return false;
            }
        }
        &self.min_level <= m.get_level() && m.get_level() <= &self.max_level
    }
}

impl Default for MessageCondition {
    fn default() -> Self {
        Self {
            component: None,
            min_level: Level::min(),
            max_level: Level::max(),
        }
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