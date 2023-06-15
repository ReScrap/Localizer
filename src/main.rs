use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;

fn decode_string(string: &str) -> Option<String> {
    let mut res = String::new();
    let mut buf = String::new();
    let mut is_parsing_hex = false;

    for c in string.chars() {
        if buf.len() == 4 {
            is_parsing_hex = false;

            let bytes = u16::from_str_radix(&buf, 16).unwrap();
            let byte_arr = [bytes];
            let decoded = String::from_utf16(&byte_arr).unwrap();

            res.push_str(&decoded);
            buf = String::new();
        }

        if !is_parsing_hex && c != char::from_u32(0x01).unwrap() {
            res.push(c);
            continue;
        }

        if c == char::from_u32(0x01).unwrap() {
            is_parsing_hex = true;
            continue;
        }

        buf.push(c);
    }
    return Some(res);
}

fn main() -> std::io::Result<()> {
    let file = File::open("encoded.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let string = &line.unwrap();
        let res = decode_string(string).unwrap();
        println!("{res}");
    }

    return Ok(());
}
