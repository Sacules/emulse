#![allow(clippy::new_without_default)]

use std::path::PathBuf;

use polodb_core::bson::doc;
use serde::{Deserialize, Serialize};

use crate::darkroom;

const IMAGE_COLLECTION: &str = "image";

pub struct Database {
    db: polodb_core::Database,
}

impl Database {
    pub fn new() -> Self {
        let db = polodb_core::Database::open_file("emulse.db").unwrap();

        Self { db }
    }

    pub fn insert_images(
        &self,
        images: &Vec<Image>,
    ) -> Result<polodb_core::results::InsertManyResult, polodb_core::Error> {
        self.db
            .collection::<Image>(IMAGE_COLLECTION)
            .insert_many(images)
    }

    pub fn get_images(&self) -> polodb_core::Result<Vec<Image>> {
        self.db.collection(IMAGE_COLLECTION).find(None)?.collect()
    }

    pub fn get_images_in_path(&self, path: PathBuf) -> polodb_core::Result<Vec<Image>> {
        self.db
            .collection(IMAGE_COLLECTION)
            .find(doc! {
                "path": path.to_string_lossy().to_string(),
            })?
            .collect()
    }

    pub fn delete_image_in_path(
        &self,
        path: PathBuf,
    ) -> Result<polodb_core::results::DeleteResult, polodb_core::Error> {
        self.db
            .collection::<Image>(IMAGE_COLLECTION)
            .delete_one(doc! {
                "path": path.to_string_lossy().to_string(),
            })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Image {
    pub path: String,
    pub uniform: darkroom::uniform::FragmentUniform,
}
