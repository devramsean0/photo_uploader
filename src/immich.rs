use serde_json::json;

use crate::environment_config;

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
        self.album = Album::new(self, album_name).unwrap();

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
        let json_album_req: serde_json::Value = serde_json::from_str(album_req.as_str())?;

        let album_id= if let Some(albums_array) = json_album_req.as_array() {
            if let Some(album_obj) = albums_array.iter().find(|a| {
                a.get("albumName").and_then(|n| n.as_str()) == Some(album_name.as_str())
            }) {
                let id = album_obj["id"].as_str()
                    .ok_or("FATAL: Album with no ID???")?;
                println!("Found album");
                id.to_string();
            } else {
                println!("Album not found, creating");
                let create_json_body = serde_json::json!({
                    "albumName": album_name,
                    "albumUsers": [{
                        "userId": immich.user_id,
                        "role": "editor"
                    }]
                });

                let create_req = immich.client
                    .post(format!("{}/albums", immich.env_config.clone().get().base_url))
                    .header("x-api-key", immich.env_config.clone().get().api_key)
                    .send()?
                    .text()?;
                let json_create_req: serde_json::Value = serde_json::from_str(create_req.as_str())?;
                let id = json_create_req["id"].as_str()
                    .ok_or("FATAL: Failed to get ID of new album");
                println!("Created new album :)");

                id.to_string()
            }
        };
        Ok(
            Album {
                name: album_name,
                id: album_id
            }
        )
    }
}

