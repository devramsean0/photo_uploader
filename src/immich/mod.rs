use crate::environment_config;

pub mod album;

pub struct Immich {
    client: reqwest::blocking::Client,
    env_config: environment_config::Config,
    pub user_id: String,
    pub album: Option<album::Album>
}

impl Immich {
    pub fn new() -> Result<Immich, Box<dyn std::error::Error>> {
        let mut env_config;
        match environment_config::Config::load_from_file() {
            Ok(config) => {
                env_config = config
            }
            Err(err) => {
                println!("Error when loading the config: {err}");
                std::process::exit(1);
            }
        }
        let client = reqwest::blocking::Client::new();
        // Go fetch user information
        let user_req = client
            .get(format!("{}/users/me", env_config.clone().get().base_url))
            .header("x-api-key", env_config.clone().get().api_key)
            .send()?
            .text()?;

        let json_user_req: serde_json::Value = serde_json::from_str(user_req.as_str())?;

        Ok(Immich {
            client,
            env_config,
            user_id: json_user_req["id"].to_string(),
            album: None
        })
    }
}