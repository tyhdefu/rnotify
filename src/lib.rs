#![cfg_attr(docsrs, feature(doc_cfg))]
//! # rnotify
//! rnotify is a binary and library for sending notifications to various services such as:
//! - Discord
//! - Email
//! - Telegram
//! - A file
//!
//! See available destinations [here](destination::kinds)
//!
//! ## Binary Usage ##
//! The rnotify binary is a simple wrapper around the library, implementing a config file and command
//! line options.
//!
//! ### Configuration File ###
//! Located in the user's home directory, .rnotify.toml is a toml file of the following structure.
//! The default configuration file is generated on the first run of the program and should
//! look something like:
//! ```toml
//! [[destinations]]
//! routing_type = "Root"
//! type = "File"
//! id = "log_file"
//! path = "C:\\Users\\name\\rnotify.log" # On windows
//! ```
//!
//! An example of a discord destination
//! ```toml
//! [[destinations]]
//! type = "Discord"
//! id = "discord_heating"
//! url = "https://discord.com/api/webhooks/..../......."
//! ```
//!
//! The default [MessageRoutingBehaviour][destination::routed_destination::MessageRoutingBehaviour] that
//! messages will go to the destination in addition to any other destinations.

pub mod message;
pub mod config;
pub mod destination;
pub mod message_router;
pub mod send_error;

#[cfg(feature = "http")]
pub mod http_util;