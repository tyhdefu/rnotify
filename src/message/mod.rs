use std::cmp::Ordering;
use std::fmt::Debug;
use message::formatted_detail::FormattedMessageDetail;
use serde::{Serialize, Deserialize};
use crate::message;
use crate::message::author::Author;
use crate::message::component::Component;

pub mod formatted_detail;
pub mod author;
pub mod component;
pub mod builder;
pub mod detail_builder;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    level: Level,
    title: Option<String>,
    message_detail: MessageDetail,
    component: Option<Component>,
    author: Author,
    unix_timestamp_millis: i64,
}

impl Message {
    pub fn new(level: Level, title: Option<String>,
               message_detail: MessageDetail, component: Option<Component>,
               author: Author, unix_timestamp_millis: i64,
    ) -> Self {
        Self {
            level,
            title,
            message_detail,
            component,
            author,
            unix_timestamp_millis
        }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    pub fn get_message_detail(&self) -> &MessageDetail {
        &self.message_detail
    }

    pub fn get_unix_timestamp_millis(&self) -> i64 {
        self.unix_timestamp_millis
    }

    pub fn get_author(&self) -> &Author {
        &self.author
    }

    pub fn get_component(&self) -> &Option<Component> {
        &self.component
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageDetail {
    Raw(String),
    Formatted(FormattedMessageDetail),
}

impl MessageDetail {
    pub fn raw(&self) -> &str {
        match &self {
            MessageDetail::Raw(raw) => raw,
            MessageDetail::Formatted(formatted) => formatted.raw()
        }
    }

    pub fn has_formatting(&self) -> bool {
        matches!(&self, MessageDetail::Formatted(_))
    }
}

impl Default for MessageDetail {
    fn default() -> Self {
        Self::Raw(String::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "binary", derive(clap::ArgEnum))]
pub enum Level {
    Info,
    Warn,
    Error,
    SelfInfo,
    SelfError,
}

impl Default for Level {
    fn default() -> Self {
        Self::Info
    }
}

impl Level {
    pub fn get_priority(&self) -> u32 {
        match &self {
            Self::Info => 1,
            Self::SelfInfo => 2,
            Self::Warn => 3,
            Self::Error => 4,
            Self::SelfError => 5,
        }
    }

    pub fn min() -> Level {
        Level::Info
    }

    pub fn max() -> Level {
        Level::SelfError
    }
}

impl PartialOrd<Self> for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}
