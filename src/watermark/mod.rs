pub mod exif;

use log::debug;

static Y_OFFSET: u32 = 100;

#[derive(Clone)]
pub struct WatermarkEngine {
    fallback_model: Option<String>, 
}

impl WatermarkEngine {
    pub fn new(fallback_model: Option<String>) -> WatermarkEngine {
        WatermarkEngine {
            fallback_model
        }
    }

    pub fn process_image(self, file_path: std::path::PathBuf, exif: exif::Exif ) {
        debug!("Constructing watermark for {}", file_path.clone().to_string_lossy().to_string());
        let mut text = Vec::new();
        if exif.model != None {
            text.push(exif.model.unwrap());
        } else {
            text.push(self.fallback_model.unwrap());
        }

        if exif.datetime != None {
            text.push(exif.datetime.unwrap());
        } else if exif.digitized_datetime != None {
            // Fallback to datetime it is digitized
            text.push(format!("Digitized: {}", exif.digitized_datetime.unwrap()));
        }

        let string_text = text.join("\n");
        debug!("Using this text for a watermark: {:#?}", text);

        let mut img = photon_rs::native::open_image(file_path.clone()).expect("File Should open");

        let render_at_y = img.get_height() - Y_OFFSET; // Start 20px from the bottom
        debug!("Rendering text {} from the bottom", render_at_y);

        photon_rs::text::draw_text(&mut img, &string_text, 10, render_at_y as i32, 32.0);
        photon_rs::native::save_image(img, format!("edited/{}", file_path.to_string_lossy().to_string()));
    }
}