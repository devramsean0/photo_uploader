use clap::{Parser, Subcommand};

mod environment_config;
mod immich;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config {
        /// Change config of application
        #[clap(short, long)]
        base_url: String,
        #[clap(short, long)]
        api_key: String
    },
    Upload {
        /// Command to do the upload process
        #[clap(short, long)]
        directory: String,
        #[clap(short, long)]
        album_name: String,
        #[clap(short, long)]
        camera_model: Option<String>
    }
}


fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Config { base_url, api_key} => {
            match environment_config::Config::new(base_url, api_key) {
                Ok(_) => {
                    println!("Config successfully written!");
                }
                Err(_) => {
                    println!("Error writing config :(");
                }
            }
        }
        Commands::Upload { directory, album_name, camera_model } => {
            match immich::Immich::new() {
                Ok(immich) => {
                    dbg!(immich.clone().user_id);
                    immich.get_album(album_name);
                }
                Err(err) => {
                    println!("Error: {err}");
                }
            }
            
        }
    }
}