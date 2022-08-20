use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use crate::destination::kinds::DestinationKind;
use crate::{DestinationConfig, MessageRoutingBehaviour};
use crate::destination::kinds::file::FileDestination;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    destinations: Vec<DestinationConfig>,
}

impl Config {
    pub fn get_destinations(&self) -> &Vec<DestinationConfig> {
        &self.destinations
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
                DestinationConfig::new(MessageRoutingBehaviour::Root, DestinationKind::File(FileDestination::new(log_path)), None)
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

pub fn fetch_config_file(verbose: bool, config_file_path: &Option<PathBuf>, default_path: &PathBuf) -> File {
    if config_file_path.is_some() {
        return File::options().read(true).open(config_file_path.as_ref().unwrap())
            .expect(&format!("Failed to open config file provided by argument for reading, {:?}", config_file_path));
    }

    let home_dir_path = get_home_dir();
    if verbose {
        println!("HomeDir: '{}'", home_dir_path);
    }

    let mut path_buf: PathBuf = home_dir_path.into();

    if !path_buf.exists() {
        panic!("Home directory does not exist!");
    }
    path_buf.push(default_path);
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

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::destination::kinds::discord::DiscordDestination;
    use super::*;

    #[test]
    fn test_mixed() {
        let s = fs::read_to_string("test/mixed.toml").expect("Failed to read file");
        let config: Config = toml::from_str(&s).expect("Failed to deserialize.");

        let file_dest = DestinationKind::File(FileDestination::new(PathBuf::from("/var/log/rnotify.log")));
        let dsc_dest = DestinationKind::Discord(DiscordDestination::new("https://discord.com/api/webhooks/11111111111111/2aaaaaaaaaaaaaaaaa".to_owned()));


        let expected_destinations = vec![
            DestinationConfig::new(MessageRoutingBehaviour::Root, file_dest, None),
            DestinationConfig::new(Default::default(), dsc_dest, None),
        ];

        assert_eq!(config.destinations, expected_destinations);
    }
}