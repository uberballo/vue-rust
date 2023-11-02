use image::Rgb;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::utils::rgb_to_hex;

#[derive(Debug)]
pub struct ColorLevels {
    pub levels: Vec<(Rgb<u8>, isize)>,
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
        println!("{:?}", vector.first().unwrap().0 .0);
        json.serialize_field(
            "colorLevels",
            &vector
                .iter()
                .map(|x| (rgb_to_hex(x.0), x.1))
                .collect::<Vec<(String, isize)>>(),
        )?;

        json.end()
    }
}
