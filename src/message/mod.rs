use std::fmt::Debug;
use clap::clap_derive::ValueEnum;
use message::formatted_detail::FormattedMessageDetail;
use crate::message;

pub mod formatted_detail;

#[derive(Debug, Clone)]
pub struct Message {
    level: Level,
    title: Option<String>,
    message_detail: MessageDetail,
    component: Option<String>,
    author: Option<String>,
    unix_timestamp_millis: i64,
}

impl Message {
    pub fn new(level: Level, title: Option<String>,
               message_detail: MessageDetail, component: Option<String>,
               author: Option<String>, unix_timestamp_millis: i64,
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

    pub fn get_author(&self) -> &Option<String> {
        &self.author
    }

    pub fn get_component(&self) -> &Option<String> {
        &self.component
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, ValueEnum, Clone)]
pub enum Level {
    Info,
    Warn,
    Error,
    SelfInfo,
    SelfError,
}
