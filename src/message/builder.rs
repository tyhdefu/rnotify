use std::time::{SystemTime, UNIX_EPOCH};
use crate::message::component::Component;
use crate::message::{Level, Message, MessageDetail};
use crate::message::author::Author;
use crate::message::detail_builder::MessageDetailBuilder;

pub struct MessageBuilder {
    level: Level,
    title: Option<String>,
    detail: MessageDetail,
    component: Option<Component>,
    author: Author,
    unix_timestamp_millis: i64,
}

impl MessageBuilder {
    pub fn new() -> Self {
        let unix_timestamp_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Expected now to be after unix epoch")
            .as_millis() as i64;

        let author = Author::base();

        Self {
            level: Default::default(),
            title: None,
            detail: Default::default(),
            component: None,
            author,
            unix_timestamp_millis,
        }
    }

    pub fn body<F>(&mut self, apply: F) -> &mut Self
        where F: FnOnce(&mut MessageDetailBuilder) {
        let mut builder = MessageDetailBuilder::new();
        apply(&mut builder);
        self.detail = builder.build();
        self
    }

    pub fn level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    pub fn title<S: ToString>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn component(&mut self, component: Component) -> &mut Self {
        self.component = Some(component);
        self
    }

    pub fn author<S: ToString>(&mut self, parts: S) -> &mut Self {
        self.author.extend(parts.to_string());
        self
    }

    pub fn timestamp(&mut self, unix_timestamp_millis: i64) -> &mut Self {
        self.unix_timestamp_millis = unix_timestamp_millis;
        self
    }

    pub fn build(self) -> Message {
        Message {
            level: self.level,
            title: self.title,
            message_detail: self.detail,
            component: self.component,
            author: self.author,
            unix_timestamp_millis: self.unix_timestamp_millis,
        }
    }
}