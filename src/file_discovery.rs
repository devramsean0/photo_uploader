use std::{fs::read_dir, path::PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub extension: String,
}

#[derive(Debug, Clone)]
pub struct Files {
    directory: String,
    pub files: Vec<File>,
}

impl Files {
    pub fn new(directory: String) -> Result<Files, Box<dyn std::error::Error>> {
        let files = read_dir(directory.clone())?
            .filter_map(|res| res.ok())
            .map(|entry| entry.path())
            .filter_map(|path| {
                if path
                    .extension()
                    .map_or(false, |ext| ext == "png" || ext == "jpg" || ext == "jpeg")
                {
                    Some(File {
                        name: path.clone().file_name()?.to_string_lossy().to_string(),
                        path: path.clone(),
                        extension: path.extension()?.to_string_lossy().to_string(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<File>>();
        Ok(Files {
            directory: directory,
            files: files,
        })
    }
}
