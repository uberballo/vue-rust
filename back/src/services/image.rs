use crate::types::ColorLevels;
use crate::utils::read_file_string;
use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

pub fn convert_and_get_levels(data: String) -> Result<ColorLevels, Box<dyn Error>> {
    let image = convert_data_url_to_jpg(data)?;
    Ok(ColorLevels {
        levels: get_color_levels(image),
    })
}

pub fn convert_and_invert_colors(data: String) -> Result<String, Box<dyn Error>> {
    let mut image = convert_data_url_to_jpg(data)?;
    image.invert();
    convert_jpg_to_base64(image)
}

//Support only base64 encoded jpeg images
fn get_image_data_from_base64(base64: String) -> Option<String> {
    match base64.contains("data:image/jpeg;base64,") {
        false => None,
        true => base64.split(",").map(String::from).last(),
    }
}
fn convert_data_url_to_jpg(data: String) -> Result<DynamicImage, Box<dyn Error>> {
    let base64_image_data = get_image_data_from_base64(data).ok_or("Invalid data")?;
    Ok(convert_base64_to_jpg(base64_image_data)?)
}
fn convert_base64_to_jpg(data: String) -> Result<DynamicImage, image::ImageError> {
    let content = general_purpose::STANDARD.decode(data).unwrap();
    image::load_from_memory_with_format(&content, image::ImageFormat::Jpeg)
}

fn get_color_levels(image: DynamicImage) -> Vec<(Rgb<u8>, isize)> {
    let mut map: HashMap<image::Rgb<u8>, isize> = HashMap::new();
    for (_, _, pixel) in image.pixels() {
        *map.entry(pixel.to_rgb()).or_default() += 1;
    }
    let mut values: Vec<(image::Rgb<u8>, isize)> = map.into_iter().collect();
    values.sort_by(|a, b| b.1.cmp(&a.1));
    values.truncate(50);
    return values;
}

fn convert_jpg_to_base64(image: DynamicImage) -> Result<String, Box<dyn Error>> {
    let mut buf = Cursor::new(vec![]);
    image.write_to(&mut buf, image::ImageOutputFormat::Jpeg(65))?;
    let content = general_purpose::STANDARD.encode(buf.get_ref());
    return Ok(format!(
        "data:image/jpeg;base64,{}",
        content.replace("\r\n", "")
    ));
}

#[test]
fn convert_jpg_to_base64_works() {
    let img = image::open("image.jpg").expect("FileNotFound");
    let content = convert_jpg_to_base64(img).unwrap();
    let text = read_file_string("img-base64.txt");
    assert_eq!(content, text);
}

#[test]
fn get_image_data_from_base64_works() {
    let data = "/9j/AAD/".to_string();
    let input = format!("data:image/jpeg;base64,{}", data);
    let result = get_image_data_from_base64(input).unwrap();
    assert_eq!(result, data);
}

#[test]
fn get_image_data_from_base64_returns_error() {
    let input = "something".to_string();
    let result = get_image_data_from_base64(input);
    assert_eq!(result, None);
}
