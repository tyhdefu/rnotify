//! # Destination List #
//! These are the available [`MessageDestinations`](crate::destination::MessageDestination)
//! Some are only available when certain features are enabled.
//! Enabling the corresponding feature will allow deserialization by serde.
//!
//! ## File ##
//! Always enabled.
//! Useful as a root destination as it is highly unlikely to fail.
//!
//! ## Discord ##
//! Feature: **discord** - enabled by default
//!
//! 1st class support for formatting.
//!
//! ## Telegram ##
//! Feature: **telegram** - enabled by default
//!
//! Reasonable support for formatting
//!
//! ## Mail ##
//! Feature: **mail**
//!
//! Reasonable support for formatting.
//!
//! ## Rust Receiver ##
//! Always enabled.
//!
//! Simple, destination that sends messages to a rust [channel](std::sync::mpsc::channel)
//!

pub mod file;
#[cfg(feature = "discord")]
#[cfg_attr(docsrs, doc(cfg(feature = "discord")))]
pub mod discord;
#[cfg(feature = "mail")]
#[cfg_attr(docsrs, doc(cfg(feature = "mail")))]
pub mod mail;
#[cfg_attr(docsrs, doc(cfg(feature = "telegram")))]
#[cfg(feature = "telegram")]
pub mod telegram;
pub mod rust_receiver;