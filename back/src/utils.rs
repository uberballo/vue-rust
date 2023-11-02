use std::fs;

use image::Rgb;

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
