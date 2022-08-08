use std::fmt::{Debug, Display};
use chrono::{DateTime, TimeZone};
use clap::clap_derive::ValueEnum;

#[derive(Debug, Clone)]
pub struct Message<TZ: TimeZone>
    where TZ::Offset: Display {
    level: Level,
    title: Option<String>,
    message_detail: String,
    component: Option<String>,
    author: Option<String>,
    timestamp: DateTime<TZ>,
}

impl<TZ: TimeZone> Message<TZ>
    where TZ::Offset: Display {
    pub fn new(level: Level, title: Option<String>,
               message_detail: String, component: Option<String>,
               author: Option<String>, timestamp: DateTime<TZ>,
    ) -> Self {
        Self {
            level,
            title,
            message_detail,
            component,
            author,
            timestamp
        }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    pub fn get_message_detail(&self) -> &String {
        &self.message_detail
    }

    pub fn get_timestamp(&self) -> &DateTime<TZ> {
        &self.timestamp
    }

    pub fn get_author(&self) -> &Option<String> {
        &self.author
    }

    pub fn get_component(&self) -> &Option<String> {
        &self.component
    }
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Level {
    Info,
    Warn,
    Error,
    SelfError,
}