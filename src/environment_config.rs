use dirs::config_dir;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use std::fs::{read_to_string, write};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new(base_url: String, api_key: String) -> SerdeResult<Config> {
        let config = Config { base_url, api_key };
        let string_config = serde_json::to_string(&config)?;

        let config_file = format!("{}/photo_uploader.json", config_dir().unwrap().display());
        debug!("Writing {:#?} to {}", string_config, config_file);
        write(config_file, string_config);

        Ok(config)
    }

    pub fn get(self) -> Self {
        self
    }

    pub fn load_from_file() -> Result<Config, std::io::Error> {
        let config_file = format!("{}/photo_uploader.json", config_dir().unwrap().display());
        let data = read_to_string(config_file.clone())?;
        let json: Config = serde_json::from_str(data.as_str())?;
        debug!("Reading {:#?} from {}", json, config_file);
        Ok(json)
    }
}
