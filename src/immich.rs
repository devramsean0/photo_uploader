use crate::environment_config;
use log::{error, info};

#[derive(Clone, Debug)]
pub struct Immich {
    client: reqwest::blocking::Client,
    env_config: environment_config::Config,
    pub user_id: String,
    pub album: Option<Album>,
}

impl Immich {
    pub fn new() -> Result<Immich, Box<dyn std::error::Error>> {
        let env_config;
        match environment_config::Config::load_from_file() {
            Ok(config) => env_config = config,
            Err(err) => {
                error!("Error when loading the config: {err}");
                std::process::exit(1);
            }
        }
        let client = reqwest::blocking::Client::new();
        // Go fetch user information
        let user_req = client
            .get(format!("{}/users/me", env_config.clone().get().base_url))
            .header("x-api-key", env_config.clone().get().api_key)
            .send()?
            .json::<serde_json::Value>()?;

        //let json_user_req: ImmichGetMyUserResponse = serde_json::from_str(user_req.as_str())?;
        Ok(Immich {
            client,
            env_config,
            user_id: user_req["id"].to_string().replace("\"", ""),
            album: None,
        })
    }

    pub fn get_album(mut self, album_name: String) -> Self {
        match Album::new(self.clone(), album_name) {
            Ok(album) => {
                self.album = Some(album);
            }
            Err(err) => {
                error!("Error getting the album: {err}");
            }
        }

        self
    }
}

#[derive(Clone, Debug)]
pub struct Album {
    id: String,
    name: String,
}

impl Album {
    pub fn new(immich: Immich, album_name: String) -> Result<Album, Box<dyn std::error::Error>> {
        let album_req = immich
            .client
            .get(format!(
                "{}/albums",
                immich.env_config.clone().get().base_url
            ))
            .header("x-api-key", immich.env_config.clone().get().api_key)
            .send()?
            .json::<Vec<serde_json::Value>>()?;
        //dbg!(json_album_req);
        let mut album_id = "";
        let mut found_album: bool = false;
        for album in album_req.iter() {
            //dbg!(album.get("albumName").unwrap().to_string());
            if album.get("albumName").unwrap().to_string() == format!("\"{}\"", album_name) {
                found_album = true;
                album_id = album.get("id").unwrap().as_str().replace("\"").unwrap();
            }
        }

        if found_album {
            info!("Found Album {} with ID: {}", album_name, album_id);
            Ok(Album {
                name: album_name,
                id: album_id.to_string(),
            })
        } else {
            info!("Album not found, creating!");
            let create_album_data = serde_json::json!({
                "albumName": album_name,
                "albumUsers": [{
                    "userId": immich.clone().user_id,
                    "role": "editor"
                }]
            });

            //dbg!(create_album_data.clone());

            let create_album_res = immich
                .client
                .post(format!(
                    "{}/albums",
                    immich.env_config.clone().get().base_url
                ))
                .header("x-api-key", immich.env_config.clone().get().api_key)
                .json(&create_album_data)
                .send()?
                .json::<serde_json::Value>()?;

            //dbg!(create_album_res.clone());
            Ok(Album {
                name: album_name,
                id: create_album_res["id"].to_string(),
            })
        }
    }
}
