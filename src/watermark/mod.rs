pub mod exif;

use log::debug;

static Y_OFFSET: u32 = 40;
static X_OFFSET: i32 = 20;

#[derive(Clone)]
pub struct WatermarkEngine {
    fallback_model: Option<String>,
}

impl WatermarkEngine {
    pub fn new(fallback_model: Option<String>) -> WatermarkEngine {
        WatermarkEngine { fallback_model }
    }

    pub fn process_image(self, file_path: std::path::PathBuf, exif: exif::Exif) {
        debug!(
            "Constructing watermark for {}",
            file_path.clone().to_string_lossy().to_string()
        );
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

        debug!("Using this text for a watermark: {:#?}", text);

        let mut img = photon_rs::native::open_image(file_path.clone()).expect("File Should open");
        if exif.orientation.unwrap() == 6 {
            debug!("Rotated image during maniuplation");
            img = photon_rs::transform::rotate(&img, 90.0);
        }
        let mut render_at_y = img.get_height() - (Y_OFFSET + (210 * text.len()) as u32); // Start 20px from the bottom
        for line in text {
            debug!("Starting text render at {} from the top", render_at_y);
            photon_rs::text::draw_text(&mut img, &line, X_OFFSET, render_at_y as i32, 200_f32);
            render_at_y = render_at_y + 190;
        }
        photon_rs::native::save_image(
            img,
            format!("edited/{}", file_path.to_string_lossy().to_string()),
        );
    }
}
