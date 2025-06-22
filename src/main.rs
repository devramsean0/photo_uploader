extern crate exif;

use clap::{Parser, Subcommand};
use simplelog::*;
use log::{info, error, debug};

mod environment_config;
mod immich;
mod file_discovery;
mod watermark;


// Selectively enable log levels based on debug enabled
#[cfg(debug_assertions)]
fn configure_logging() {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
}

#[cfg(not(debug_assertions))]
fn configure_logging() {
    let _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
}

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
    configure_logging();
    let args = Args::parse();
    match args.cmd {
        Commands::Config { base_url, api_key} => {
            match environment_config::Config::new(base_url, api_key) {
                Ok(_) => {
                    info!("Config successfully written!");
                }
                Err(err) => {
                    error!("Error writing config: {err}");
                }
            }
        }
        Commands::Upload { directory, album_name, camera_model } => {
            match immich::Immich::new() {
                Ok(immich) => {
                    immich.get_album(album_name);
                }
                Err(err) => {
                    error!("Error connecting to immich: {err}");
                }
            }
            match file_discovery::Files::new(directory) {
                Ok(files) => {
                    debug!("{:#?}", files.clone());
                    for file in files.files {
                        info!("Processing file: {}", file.path.to_string_lossy().to_string());
                        watermark::exif::Exif::extract(file.path);
                    }
                }
                Err(err) => {
                    error!("Error discovering files: {err}");
                }
            }
        }
    }
}