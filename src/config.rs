use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use crate::destination::routed_destination::RoutedDestination;
use crate::destination::kinds::file::FileDestination;
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message_router::RoutingInfo;

const CONFIG_FILE_NAME: &str = ".rnotify.toml";

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
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        let log_path = "/var/log/rnotify.log".into();

        #[cfg(target_os = "windows")]
        let log_path = {
            let mut app_data: PathBuf = std::env::var("AppData")
                .expect("Failed to find appdata environment variable, cannot create default configuration file, try creating it manually.")
                .into();
            app_data.push("rnotify");
            app_data.push("rnotify.log");
            app_data
        };
        Self {
            destinations: vec![
                SerializableRoutedDestination::create("file_log".to_owned(), FileDestination::new(log_path), RoutingInfo::root()),
            ]
        }
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

pub fn fetch_config_file(verbose: bool, config_file_path: &Option<PathBuf>) -> File {
    if config_file_path.is_some() {
        return File::options().read(true).open(config_file_path.as_ref().unwrap())
            .expect(&format!("Failed to open config file provided by argument for reading, {:?}", config_file_path));
    }

    let path_buf = get_default_config_path();
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

        let string = toml::to_string(&Config::default()).expect("Failed to serialize default config.");
        file.write_all(string.as_bytes()).expect("Failed to write default config file to config file.");
        println!("Created default config file");
    }

    File::options()
        .read(true)
        .open(&path_buf).expect("Failed to open config file for reading.")
}

#[cfg(target_os = "windows")]
const HOME_DIR_ENVIRONMENT_VARIABLE: &str = "USERPROFILE";
#[cfg(target_os = "linux")]
const HOME_DIR_ENVIRONMENT_VARIABLE: &str = "HOME";

fn get_home_dir() -> String {
    std::env::var(HOME_DIR_ENVIRONMENT_VARIABLE)
        .expect("Failed to find homedir, in order to find config file, try setting the environment variable.")
}

pub fn get_default_config_path() -> PathBuf {
    let mut path: PathBuf = get_home_dir().into();
    path.push(CONFIG_FILE_NAME);
    path
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