
use image::{Rgb, ImageFormat, DynamicImage};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::utils::rgb_to_hex;

#[derive(Debug)]
pub struct ColorLevels {
    pub levels: Vec<(Rgb<u8>, isize)>,
}

#[derive(Serialize, Debug)]
pub struct ColorLevel {
    pub color: String,
    pub rgb: [u8;3],
    pub count: isize
}

impl ColorLevel {
    fn new(rgb: Rgb<u8>, count: isize) -> ColorLevel {
        ColorLevel { color: rgb_to_hex(rgb), rgb: rgb.0, count: count }
    }
}

#[derive(Deserialize, Debug)]
pub struct UploadData {
    pub data: String,
}

// We need to create a custom serializer due to Rgba<u8> type.
impl Serialize for ColorLevels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vector = &self.levels;
        let mut json = serializer.serialize_struct("colors", vector.len())?;
        let colors: Vec<ColorLevel> = vector.iter().map(|x| ColorLevel::new(x.0, x.1)).collect();
        json.serialize_field(
            "colorLevels",&colors
        )?;
        json.end()
    }
}

pub struct DataUrl {
    pub format: ImageFormat,
    pub data: String
}


// DynamicImage doesn't contain any information about the format.
pub struct ImageWithFormat {
    pub image: DynamicImage,
    pub format: ImageFormat
}