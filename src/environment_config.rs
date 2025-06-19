use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use std::fs::{write, read_to_string};
use dirs::config_dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: String
}

impl Config {
    pub fn new(base_url: String, api_key: String) -> SerdeResult<Config> {
        let config = Config {
            base_url,
            api_key
        };
        let string_config = serde_json::to_string(&config)?;

        let config_file = format!("{}/photo_uploader.json", config_dir().unwrap().display());
        println!("Writing to {}", config_file);
        write(config_file, string_config);

        Ok(config)
    }

    pub fn get(self) -> Self {
        self
    }

    pub fn load_from_file() -> Result<Config, std::io::Error> {
        let config_file = format!("{}/photo_uploader.json", config_dir().unwrap().display());
        let data = read_to_string(config_file)?;

        let json: Config = serde_json::from_str(data.as_str())?;

        Ok(json)
    }
}