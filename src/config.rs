use crate::logger;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub address: String,
    pub root_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3550,
            address: String::from("0.0.0.0"),
            root_dir: String::from("."),
        }
    }
}

pub fn load(filename: &str) -> Config {
    logger::log(&format!("Loading config from {}...", filename));

    match fs::read_to_string(filename) {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
            panic!("{}", logger::format_msg(&format!("Failed to parse config: {}", e)))
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
    }
}
