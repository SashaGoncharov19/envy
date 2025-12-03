use crate::logger;

use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub port: u16,
    pub address: String,
    pub root_dir: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3550,
            address: String::from("0.0.0.0"),
            root_dir: String::from("."),
        }
    }
}

pub fn load(filename: &str) {
    logger::log(&format!("Loading config from {}...", filename));

    let config = match fs::read_to_string(filename) {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
            panic!(
                "{}",
                logger::format_msg(&format!("Failed to parse config: {}", e))
            )
        }),
        Err(_) => {
            let config = Config::default();
            let toml_string = toml::to_string_pretty(&config).unwrap();

            match fs::write(filename, &toml_string) {
                Ok(_) => logger::log("Config file not found. Created default config.toml"),
                Err(e) => logger::log(&format!("[ERROR] Unable to create config file: {}", e)),
            };

            config
        }
    };

    CONFIG
        .set(config)
        .expect("Config has already been initialized!");
}

pub fn get() -> &'static Config {
    CONFIG.get().unwrap()
}
