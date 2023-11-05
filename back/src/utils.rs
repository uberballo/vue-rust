use std::{fs, error::Error };

use fancy_regex::Regex;
use image::{Rgb, ImageFormat};

use crate::types::{DataUrl};

fn number_to_hex(number: u8) -> String {
    let hex = format!("{:x}", number);
    match hex.len() {
        1 => format!("0{}", hex),
        _ => hex,
    }
}

pub fn rgb_to_hex(rgb: Rgb<u8>) -> String {
    format!("#{}", rgb.0.map(|x| number_to_hex(x)).join(""))
}

#[test]
fn rbg_to_hex_works() {
    let input: Rgb<u8> = Rgb([100, 100, 100]);
    let result = rgb_to_hex(input);
    assert_eq!(result, "#646464")
}

pub fn read_file_string(filename: &str) -> String {
    fs::read_to_string(filename).expect("Should have been able to read the file")
}

#[test]
fn read_file_works() {
    let text = read_file_string("resources/read-file-string.txt");
    let expected = "abc123".to_string();
    assert_eq!(text, expected);
}

pub fn get_data_url_format_and_data(data_url: String) -> Result<DataUrl, Box<dyn Error>> {
    Ok(DataUrl {
        format: get_data_url_format(&data_url)?,
        data: get_data_url_data(&data_url)?
    })
}

fn get_data_url_format(data_url: &str) -> Result<ImageFormat ,&str> {
    let format_string = get_data_url_mime_type(data_url)?.to_owned();
    ImageFormat::from_mime_type(format_string).ok_or("Invalid mime type")
}

fn get_data_url_mime_type(data_url: &str) -> Result<&str ,&str>{
    let re = Regex::new(r"(?<=data:)(.*)(?=;)").unwrap();
    let result = re.captures(data_url).expect("Failed to capture");
    let captures = result.expect("No match found");
    let group = captures.get(1).expect("No group");
    Ok(group.as_str())
}

fn get_data_url_data(data_url: &str) -> Result<String, &str>{
    let re = Regex::new(r"(?<=base64,)(.*)").unwrap();
    let result = re.captures(data_url).expect("Failed to capture");
    let captures = result.expect("No match found");
    let group = captures.get(1).expect("No group");
    Ok(group.as_str().to_owned())

}

#[test]
fn get_data_url_mime_type_works() {
    let input = "data:image/jpeg;base64";
    let result = get_data_url_format(input).unwrap();
    assert_eq!(result, ImageFormat::Jpeg)
}

#[test]
fn get_data_url_data_works() {
    let input = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAgAAAQABAAD/w";
    let expected = "/9j/4AAQSkZJRgABAgAAAQABAAD/w";
    let result = get_data_url_data(input).unwrap();
    assert_eq!(result, expected)
}