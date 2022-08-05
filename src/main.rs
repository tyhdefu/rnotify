mod config;
mod destinations;
mod message;

use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use crate::message::{Level, Message};

const CONFIG_FILE_NAME: &str = ".rnotify.toml";

fn main() {
    let timestamp = Instant::now();
    let cli: Cli = Cli::parse();
    let config = {
        let file = config::fetch_config_file(&cli);
        config::read_config_file(file)
    };

    let message_detail = {
        if cli.message.is_some() {
            cli.message.unwrap()
        }
        else {
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
    #[clap(short, long, value_enum, default_value_t=Level::Info)]
    level: Level,
    #[clap(short, long)]
    title: Option<String>,
    #[clap(short, long)]
    component: Option<String>,
    #[clap(short, long)]
    author: Option<String>,
}
