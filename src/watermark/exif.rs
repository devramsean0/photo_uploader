use std::fs;
use std::path::{Path, PathBuf};
use exif::{Reader, Tag, In};
use log::debug;


pub struct Exif {
    pub model: Option<String>,
    pub datetime: Option<String>,
    pub digitized_datetime: Option<String>
}

impl Exif {
    pub fn extract(file_path: PathBuf) -> Result<Exif, Box<dyn std::error::Error>> {
        debug!("Extracting exif data from {}", file_path.clone().to_string_lossy().to_string());
        let file = std::fs::File::open(file_path)?;
        let mut buffer_reader = std::io::BufReader::new(&file);
        let exif_reader = Reader::new();
        let exif = exif_reader.read_from_container(&mut buffer_reader)?;

        let mut exif_struct = Exif {
            model: None,
            datetime: None,
            digitized_datetime: None
        };

        match exif.get_field(Tag::Model, In::PRIMARY) {
            Some(model) => {
                exif_struct.model = Some(model.display_value().with_unit(&exif).to_string().replace("\"",""));
                debug!("Extracted Camera Model: {}", exif_struct.model.clone().unwrap());
            }
            None => {
                debug!("Camera Model is missing from exif")
            }
        }

        match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
            Some(datetime) => {
                exif_struct.datetime = Some(datetime.display_value().with_unit(&exif).to_string());
                debug!("Extracted DateTime: {}", exif_struct.datetime.clone().unwrap());
            }
            None => {
                debug!("DateTime is missing from exif")
            }
        }

        match exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
            Some(datetime) => {
                exif_struct.digitized_datetime = Some(datetime.display_value().with_unit(&exif).to_string());
                debug!("Extracted Digitized DateTime: {}", exif_struct.digitized_datetime.clone().unwrap());
            }
            None => {
                debug!("Digitized DateTime is missing from exif")
            }
        }
        //dbg!(file_path.to_string_lossy().to_string(), metadata.get_tag(&ExifTag::ImageHeight(vec![])).next());

        Ok(exif_struct)
    }
}