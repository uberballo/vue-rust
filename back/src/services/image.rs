use crate::types::{ColorLevels, DataUrl};
use crate::utils::{get_data_url_format_and_data, read_file_string};
use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, GenericImageView, ImageOutputFormat, Pixel, Rgb};
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

pub fn convert_and_get_levels(data: String) -> Result<ColorLevels, Box<dyn Error>> {
    let image = convert_data_url_to_image(data)?;
    Ok(ColorLevels {
        levels: get_color_levels(image),
    })
}

pub fn convert_and_invert_colors(data: String) -> Result<String, Box<dyn Error>> {
    let data_url = get_data_url_format_and_data(data)?;
    let output_format = ImageOutputFormat::from(data_url.format);
    let mime_type = data_url.format.to_mime_type();
    let mut image = convert_base64_to_image(data_url)?;
    image.invert();
    convert_jpg_to_base64(image, output_format, mime_type)
}

fn convert_data_url_to_image(data: String) -> Result<DynamicImage, Box<dyn Error>> {
    let data_url = get_data_url_format_and_data(data)?;
    Ok(convert_base64_to_image(data_url)?)
}

fn convert_base64_to_image(data_url: DataUrl) -> Result<DynamicImage, image::ImageError> {
    let content = general_purpose::STANDARD.decode(data_url.data).unwrap();
    image::load_from_memory_with_format(&content, data_url.format)
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

fn convert_jpg_to_base64(
    image: DynamicImage,
    output_format: ImageOutputFormat,
    mime_type: &str,
) -> Result<String, Box<dyn Error>> {
    let mut buf = Cursor::new(vec![]);
    match output_format {
        ImageOutputFormat::Jpeg(x) => {
            image.write_to(&mut buf, image::ImageOutputFormat::Jpeg(x))?
        }
        _ => image.write_to(&mut buf, output_format)?,
    }
    let content = general_purpose::STANDARD.encode(buf.get_ref());
    return Ok(format!(
        "data:{};base64,{}",
        mime_type,
        content.replace("\r\n", "")
    ));
}

#[test]
fn convert_jpg_to_base64_works() {
    let img = image::open("image.jpg").expect("FileNotFound");
    let content = convert_jpg_to_base64(img, ImageOutputFormat::Jpeg(65), "Jpeg").unwrap();
    let text = read_file_string("img-base64.txt");
    assert_eq!(content, text);
}
