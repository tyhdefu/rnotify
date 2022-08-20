use std::collections::HashMap;
use std::fmt::Debug;
use senderror::SendErrors;
use crate::config::Config;
use crate::destination::config::{DestinationConfig, MessageRoutingBehaviour};
use crate::message::{Level, Message};

pub mod message;
pub mod config;
pub mod destination;
pub mod error;

#[cfg(feature = "http")]
pub mod http_util;
mod senderror;

/// Send a message to all destinations as specified by the config
pub fn send_message(message: Message, config: &Config) -> Result<(), SendErrors> {

    let destinations = config.get_destinations();

    let mut errors = vec![];

    let mut sent_to_non_root_dest = false;

    for (i, dest) in destinations.iter().enumerate()
        .filter(|(_i, dest)| dest.get_routing_type() != &MessageRoutingBehaviour::Drain)
        .filter(|(_i, dest)| dest.should_receive(&message)) {

        match dest.send(&message) {
            Ok(()) => {
                if !dest.is_root() {
                    sent_to_non_root_dest = true;
                }
            }
            Err(err) => errors.push((err, i, dest)),
        };
    }

    if !sent_to_non_root_dest {
        // Find a drain.
        for (i, dest) in destinations.iter().enumerate()
            .filter(|(_i, dest)| dest.get_routing_type() == &MessageRoutingBehaviour::Drain)
            .filter(|(_i, dest)| dest.should_receive(&message)) {

            if let Err(err) = dest.send(&message) {
                errors.push((err, i, dest))
            }
        }
    }

    if errors.is_empty() {
        return Ok(());
    }

    if !destinations.iter().any(|dest| dest.is_root()) {
        let errors = errors.into_iter()
            .map(|(err, i, dest)| {
                let dest = dest.to_owned();
                (dest.to_owned(), i, err, HashMap::new())
            })
            .collect();
        return Err(SendErrors::new(vec![], message, errors));
    }

    let root: Vec<_> = destinations.iter()
        .filter(|dest| dest.is_root())
        .map(|dest| dest.to_owned())
        .collect();

    let errors = errors.into_iter().map(|(err, i, dest)| {
        let message = {
            SendError::from(&err, i, dest, &message)
        }.to_message();
        // Send any send errors to root destinations.
        let root_errors_indices = root.iter().enumerate()
            .map(|(i, dest)| (i, dest.send(&message)))
            .filter(|(_i, result)| result.is_err())
            .map(|(i, result)| (i, result.unwrap_err()))
            .collect();
        (dest.to_owned(), i, err, root_errors_indices)
    }).collect();

    Err(SendErrors::new(root, message, errors))
}

#[derive(Debug)]
pub struct SendError<'a> {
    err: &'a Box<dyn std::error::Error>,
    index: usize,
    item_string: String,
    message: &'a Message,
}

impl<'a> SendError<'a> {
    pub fn to_message(&self) -> Message {
        Message::new(Level::SelfError,
                     Some(format!("Failed to send notification to destination {}", self.index)),
                     message::MessageDetail::Raw(format!("Rnotify failed to send a message {:?} to destination '{}'. Error: '{}' A notification has been sent here because this is configured as a root logger.",
                                                         self.message, self.item_string, self.err)),
                     None,
                     None,
                     self.message.get_unix_timestamp_millis().clone(),
        )
    }
}

impl<'a> SendError<'a> {
    pub fn from(err: &'a Box<dyn std::error::Error>, i: usize, item: &DestinationConfig, message: &'a Message) -> Self
    {
        Self {
            err,
            index: i,
            item_string: serde_json::to_string(item).unwrap_or_else(|_| format!("{:?}", item)),
            message,
        }
    }
}
