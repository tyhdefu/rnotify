mod config;
mod destinations;
mod message;
mod destination_kind;
mod destination_config;

use std::fmt::{Debug, Display};
use std::io::Read;
use std::path::PathBuf;
use chrono::{Local, TimeZone};
use clap::Parser;
use crate::destination_config::DestinationConfig;
use crate::message::{Level, Message};

const CONFIG_FILE_NAME: &str = ".rnotify.toml";

fn main() {
    // TODO: Allow configuration of timezone.
    let timestamp = Local::now();
    let cli: Cli = Cli::parse();
    let config = {
        let file = config::fetch_config_file(&cli);
        config::read_config_file(file)
    };

    let message_detail = {
        if cli.message.is_some() {
            cli.message.unwrap()
        } else {
            if cli.verbose {
                println!("Reading stdin.");
            }
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).expect("Error reading stdin.");
            s
        }
    };


    // Construct message.
    let message = Message::new(
        cli.level,
        cli.title,
        message_detail,
        cli.component,
        cli.author,
        timestamp,
    );
    if cli.verbose {
        println!("Message: {:?}", message);
    }

    let destinations = config.get_destinations();

    let errors: Vec<_> = destinations.iter()
        .enumerate()
        .filter_map(|(i, dest)| dest.send(&message).err()
            .map(|err| SendError::from(err, i, dest, message.clone())))
        .collect();

    if errors.is_empty() {
        return;
    }

    if !destinations.iter().any(|dest| dest.is_root()) {
        panic!("No root loggers configured and errors sending to destination(s) occurred. Please setup a root destination. Errors: {:?}", errors);
    }

    let mut any_failed = false;
    for send_error in errors {
        let message = send_error.to_message();
        // Send any send errors to root destinations.
        destinations.iter()
            .filter(|dest| dest.is_root())
            .for_each(|dest| {
            match dest.send(&message) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to send error '{:?}' to root destination, these should be destinations that never fail. Error: {}, Root logger destination: {:?}", message, err, dest);
                    any_failed = true;
                }
            }
        })
    }
    if any_failed {
        panic!("Failed to send self errors to root loggers, check above lines for details.")
    }
}

#[derive(Debug)]
struct SendError<TZ: TimeZone>
where TZ::Offset: Display
{
    err: Box<dyn std::error::Error>,
    index: usize,
    item_string: String,
    message: Message<TZ>,
}

impl<TZ: TimeZone + Debug> SendError<TZ>
    where TZ::Offset: Display {

    pub fn to_message(&self) -> Message<TZ> {
        Message::new(Level::SelfError,
                     Some(format!("Failed to send notification to destination {}", self.index)),
                     format!("Rnotify failed to send a message {:?} to destination '{}'. Error: '{}' A notification has been sent here because this is configured as a root logger.",
                             self.message, self.item_string, self.err),
                     None,
                     None,
                     self.message.get_timestamp().clone(),
        )
    }
}

impl<TZ: TimeZone> SendError<TZ>
    where TZ::Offset: Display {
    pub fn from(err: Box<dyn std::error::Error>, i: usize, item: &DestinationConfig, message: Message<TZ>) -> Self
    {
        Self {
            err,
            index: i,
            item_string: serde_json::to_string(item).unwrap_or_else(|_| format!("{:?}", item)),
            message,
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, value_parser)]
    config_file: Option<PathBuf>,
    #[clap(short, long)]
    verbose: bool,
    #[clap(long)]
    dry_run: bool,

    // (Or read from stdin)
    #[clap(short, long)]
    message: Option<String>,
    #[clap(short, long, value_enum, default_value_t = Level::Info)]
    level: Level,
    #[clap(short, long)]
    title: Option<String>,
    #[clap(short, long)]
    component: Option<String>,
    #[clap(short, long)]
    author: Option<String>,
}
