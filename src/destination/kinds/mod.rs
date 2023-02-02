pub mod file;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "mail")]
pub mod mail;
#[cfg(feature = "telegram")]
pub mod telegram;
#[cfg(test)]
pub mod rust_receiver;