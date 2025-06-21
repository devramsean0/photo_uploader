use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Exif {
    camera_name: Option<String>,
    date: Option<String>,
    time: Option<String>
}

impl Exif {
    pub fn extract(file_path: PathBuf) -> Result<Exif, Box<dyn std::error::Error>> {
        let metadata = &Metadata::new_from_path(&file_path)?;

        //dbg!(file_path.to_string_lossy().to_string(), metadata.get_tag(&ExifTag::ImageHeight(vec![])).next());

        Ok(Exif {
            camera_name: None,
            date: None,
            time: None
        })
    }
}