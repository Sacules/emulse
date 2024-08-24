use rayon::prelude::*;
use std::{io, path::PathBuf, sync::Arc};

use image::DynamicImage;

const VALID_FORMATS: [&str; 4] = ["jpg", "jpeg", "bmp", "tif"];

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub data: DynamicImage,
    pub path: PathBuf,
}

pub fn load_from_dir(path: PathBuf) -> Result<Vec<Arc<Image>>, io::Error> {
    let iter = path
        .read_dir()?
        .par_bridge()
        .into_par_iter()
        .filter(|file| match file.as_ref().unwrap().path().extension() {
            Some(ext) => VALID_FORMATS
                .iter()
                .any(|format| *format == ext.to_ascii_lowercase()),
            None => false,
        });
    let mut out: Vec<Arc<Image>> = iter
        .map(|entry| {
            let entry_path = entry.unwrap().path();
            match image::open(entry_path.clone()) {
                Ok(data) => Arc::new(Image {
                    data,
                    path: entry_path,
                }),
                Err(err) => {
                    panic!("{err}")
                }
            }
        })
        .collect();
    out.sort_by(|a, b| {
        a.path
            .to_string_lossy()
            .to_string()
            .to_lowercase()
            .cmp(&b.path.to_string_lossy().to_string().to_lowercase())
    });

    Ok(out)
}
