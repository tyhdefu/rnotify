use std::io::Read;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::Parser;
use rnotifylib::{config, message, send_message};
use rnotifylib::message::{Level, Message, MessageDetail};
use rnotifylib::message::author::Author;

const CONFIG_FILE_NAME: &str = ".rnotify.toml";

fn main() {
    // TODO: Allow configuration of timezone.
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Current time is before the unix epoch!")
        .as_millis();
    let cli: Cli = Cli::parse();

    let config = {
        let file = config::fetch_config_file(cli.verbose, &cli.config_file, &PathBuf::from(CONFIG_FILE_NAME));
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


    let message_detail = if cli.formatted {
        MessageDetail::Formatted(message::formatted_detail::parse_raw_to_formatted(&message_detail))
    } else {
        MessageDetail::Raw(message_detail)
    };

    let author = Author::parse(cli.author.unwrap_or("".to_owned()));

    // Construct message.
    let message = Message::new(
        cli.level,
        cli.title,
        message_detail,
        cli.component.as_deref().map(|s| s.into()),
        author,
        timestamp as i64,
    );

    if cli.verbose {
        println!("Message: {:?}", message);
    }

    let result = send_message(message, &config);
    if result.is_ok() {
        return;
    }
    let err = result.unwrap_err();
    eprintln!("Failed to send message to one or more destination");
    eprintln!("{}", err);
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

    #[clap(short, long)]
    formatted: bool,
}
