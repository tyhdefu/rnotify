use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use crate::destination::routed_destination::RoutedDestination;
use crate::destination::kinds::file::FileDestination;
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message_router::RoutingInfo;

const CONFIG_FILE_NAME: &str = "rnotify.toml";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    destinations: Vec<SerializableRoutedDestination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableRoutedDestination {
    id: String,
    // Whether errors with sending notifications will be reported to this destination.
    #[serde(flatten)]
    destination: Box<dyn SerializableDestination>,
    #[serde(flatten)]
    routing_info: RoutingInfo,
}

impl SerializableRoutedDestination {

    pub fn new(id: String, destination: Box<dyn SerializableDestination>, routing_info: RoutingInfo) -> Self {
        Self {
            id,
            destination,
            routing_info,
        }
    }

    pub fn create<D: SerializableDestination + 'static>(id: String, destination: D, routing_info: RoutingInfo) -> Self {
        Self {
            id,
            destination: Box::new(destination),
            routing_info
        }
    }
}

impl RoutedDestination for SerializableRoutedDestination {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_destination(&self) -> &dyn MessageDestination {
        self.destination.as_message_destination()
    }

    fn get_routing_info(&self) -> &RoutingInfo {
        &self.routing_info
    }
}

impl Config {
    pub fn get_destinations(&self) -> &Vec<SerializableRoutedDestination> {
        &self.destinations
    }

    pub fn take_destinations(self) -> Vec<SerializableRoutedDestination> {
        self.destinations
    }

    fn try_default() -> Result<Self, String> {
        let log_path = dirs::state_dir()
            .or_else(|| dirs::home_dir());

        if log_path.is_none() {
            return Err("Failed to get state directory - if you're on linux, is $HOME set?".to_owned());
        }
        let mut log_path = log_path.unwrap();
        log_path.push("rnotify.log");

        Ok(Self {
            destinations: vec![
                SerializableRoutedDestination::create("file_log".to_owned(), FileDestination::new(log_path), RoutingInfo::root()),
            ]
        })
    }
}

pub fn read_config_file(mut file: File) -> Config {
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Failed to read config file.");
    match toml::from_str(&s) {
        Ok(c) => c,
        Err(err) => panic!("Error parsing config file:{}", err),
    }
}

pub fn fetch_config_file(verbose: bool, config_file_path_override: &Option<PathBuf>) -> Result<File, String> {
    if config_file_path_override.is_some() {
        return File::options().read(true).open(config_file_path_override.as_ref().unwrap())
            .map_err(|e| format!("Failed to open config file {:?} provided by argument for reading: {}", config_file_path_override, e));
    }

    let config_dir = dirs::config_dir();
    if config_dir.is_none() {
        return Err("Failed to find config directory - if you're on linux, is $HOME set?".to_owned());
    }
    let mut path_buf = config_dir.unwrap();
    path_buf.push(CONFIG_FILE_NAME);

    if verbose {
        println!("Using config file path: {}", &path_buf.display());
    }

    if !path_buf.exists() {
        println!("Config file doesn't exist, creating it ({})", &path_buf.display());
        let mut file = File::options()
            .create_new(true)
            .write(true)
            .open(&path_buf)
            .expect("Failed to create new config file to write default config.");

        let default_config = Config::try_default().expect("Failed to create default config");
        let string = toml::to_string(&default_config)
            .expect("Failed to serialize default config.");
        file.write_all(string.as_bytes()).expect("Failed to write default config file to config file.");
        println!("Created default config file");
    }

    File::options()
        .read(true)
        .open(&path_buf)
        .map_err(|e| format!("Failed to open config file for reading: {}", e))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::destination::kinds::discord::DiscordDestination;
    use crate::destination::routed_destination::{MessageRoutingBehaviour, RoutedDestinationBase};
    use super::*;

    #[test]
    fn test_mixed() {
        let s = fs::read_to_string("test/mixed.toml").expect("Failed to read file");
        let config: Config = toml::from_str(&s).expect("Failed to deserialize.");

        let file_dest = FileDestination::new(PathBuf::from("/var/log/rnotify.log"));
        let dsc_dest = DiscordDestination::new("https://discord.com/api/webhooks/11111111111111/2aaaaaaaaaaaaaaaaa".to_owned());


        let log_file = RoutedDestinationBase::create("log_file".to_owned(), file_dest, RoutingInfo::root());
        let dsc = RoutedDestinationBase::create("discord_destination".to_owned(), dsc_dest, RoutingInfo::of(MessageRoutingBehaviour::Additive));

        assert_eq!(config.destinations[0].get_id(), log_file.get_id());
        assert_eq!(config.destinations[0].get_routing_info(), log_file.get_routing_info());

        assert_eq!(config.destinations[1].get_id(), dsc.get_id());
        assert_eq!(config.destinations[1].get_routing_info(), dsc.get_routing_info());
    }
}