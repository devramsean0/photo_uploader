use clap::builder::TypedValueParser;
use serde_json::json;

use crate::environment_config;

#[derive(Clone)]
pub struct Immich {
    client: reqwest::blocking::Client,
    env_config: environment_config::Config,
    pub user_id: String,
    pub album: Option<Album>
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

        let json_user_req: ImmichGetMyUserResponse = serde_json::from_str(user_req.as_str())?;

        Ok(Immich {
            client,
            env_config,
            user_id: json_user_req.id.unwrap(),
            album: None
        })
    }

    pub fn get_album(mut self, album_name: String) -> Self {
        Album::new(self.clone(), album_name).unwrap();

        self
    }
}


// Users Type
#[derive(serde::Deserialize)]
struct ImmichGetMyUserResponse {
    #[serde(rename = "avatarColor")]
    avatar_color: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
    #[serde(rename = "deletedAt")]
    deleted_at: Option<String>,
    email: Option<String>,   
    id: Option<String>,
    #[serde(rename = "isAdmin")]
    is_admin: Option<bool>,
    license: Option<ImmichLicenseObject>,
    name: Option<String>,
    #[serde(rename = "oauthId")]
    oauth_id: Option<String>,
    #[serde(rename = "profileChangedAt")]
    proffile_changed_at: Option<String>,
    #[serde(rename = "profileImagePath")]
    profile_image_path: Option<String>,
    #[serde(rename = "quotaSizeInBytes")]
    quota_size_in_bytes: Option<i64>,
    #[serde(rename = "quotaUsageInBytes")]
    quota_usage_in_bytes: Option<i64>,
    status: Option<String>,
    #[serde(rename = "storageLabel")]
    storage_label: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
}

#[derive(serde::Deserialize)]
struct ImmichLicenseObject {
    #[serde(rename = "activatedAt")]
    activated_at: Option<String>,
    #[serde(rename = "activationKey")]
    activation_key: Option<String>,
    #[serde(rename = "licenseKey")]
    license_key: Option<String>,
}

#[derive(Clone)]
pub struct Album {
    id: String,
    name: String,
}


impl Album {
    pub fn new(immich: Immich, album_name: String) -> Result<Album, Box<dyn std::error::Error>> {
        let album_req = immich.client
            .get(format!("{}/albums", immich.env_config.clone().get().base_url))
            .header("x-api-key", immich.env_config.clone().get().api_key)
            .send()?
            .text()?;
        let json_album_req: Vec<serde_json::Value> = serde_json::from_str(album_req.as_str())?;
        //dbg!(json_album_req);
        let mut album_id = "";
        let mut found_album: bool = false;
        for album in json_album_req.iter() {
            //dbg!(album.get("albumName").unwrap().to_string());
            if album.get("albumName").unwrap().to_string() == format!("\"{}\"", album_name) {
                found_album = true;
                album_id = album.get("id").unwrap().as_str().replace("\"").unwrap();
            }
        }

        if (found_album) {
            println!("Found Album {} with ID: {}", album_name, album_id);
            Ok(Album {
                name: album_name,
                id: album_id.to_string()
            })
        } else {
            println!("Album not found, creating!");
            let create_album_data = serde_json::json!({
                "albumName": album_name,
                "albumUsers": [{
                    "userId": immich.clone().user_id,
                    "role": "editor"
                }]
            });

            let create_album_res = immich.client
                .post(format!("{}/albums", immich.env_config.clone().get().base_url))
                .header("x-api-key", immich.env_config.clone().get().api_key)
                .json(&create_album_data)
                .send()?
                .text()?;

            Ok(Album {
                name: "".to_string(),
                id: "".to_string()
            })
        }
    }
}

