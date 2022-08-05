use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use crate::{Cli, CONFIG_FILE_NAME};
use crate::destinations::file::FileDestination;
use crate::destinations::MessageDestination;

#[cfg(feature = "discord")]
use crate::destinations::discord::DiscordDestination;

#[derive(Serialize, Deserialize)]
pub struct Config {
    destinations: Vec<DestinationConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct DestinationConfig {
    // Whether errors with sending notifications will be reported to this destination.
    root: bool,
    #[serde(flatten)]
    dest_type: DestinationKind,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DestinationKind {
    File(FileDestination),
    #[cfg(feature = "discord")]
    Discord(DiscordDestination),
}

impl DestinationKind {
    pub fn get_destination<T>(&self) -> &dyn MessageDestination {
        match &self {
            DestinationKind::File(dest) => dest,
            #[cfg(feature = "discord")]
            DestinationKind::Discord(dest) => dest,
        }
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
                DestinationConfig { root: true, dest_type: DestinationKind::File(FileDestination::new(log_path)) }
            ]
        }
    }
}

pub fn read_config_file(mut file: File) -> Config {
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Failed to read config file.");
    toml::from_str(&s).expect("Error parsing config file.")
}

pub fn fetch_config_file(cli: &Cli) -> File {
    if cli.config_file.is_some() {
        return File::options().read(true).open(cli.config_file.as_ref().unwrap())
            .expect(&format!("Failed to open config file provided by argument for reading, {:?}", cli.config_file));
    }

    let home_dir_path = get_home_dir();
    if cli.verbose {
        println!("HomeDir: '{}'", home_dir_path);
    }

    let mut path_buf: PathBuf = home_dir_path.into();

    if !path_buf.exists() {
        panic!("Home directory does not exist!");
    }
    path_buf.push(CONFIG_FILE_NAME);
    println!("Using config file path: {}", &path_buf.display());

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