use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::string::String;

#[derive(Debug)]
pub struct Config {
    pub in_file_path: PathBuf,
    pub out_file_path: Option<PathBuf>,
    pub tostd: bool,
}

pub fn decode_string(string: &str) -> Option<String> {
    let mut res = String::new();
    let mut buf = String::new();
    let mut is_parsing_hex = false;

    for c in string.chars() {
        if c == char::from_u32(0x01).unwrap() {
            is_parsing_hex = true;
            continue;
        }

        if !is_parsing_hex {
            res.push(c);
            continue;
        }

        buf.push(c);

        if buf.len() == 4 {
            is_parsing_hex = false;

            let bytes = u16::from_str_radix(&buf, 16).unwrap();
            let byte_arr = [bytes];
            let decoded = String::from_utf16(&byte_arr).unwrap();

            res.push_str(&decoded);
            buf = String::new();
        }
    }

    return Some(res);
}

pub fn encode_string(string: &str) -> Option<String> {
    let mut res = String::new();

    for c in string.chars() {
        // NOTE: didn't found rust analog of pythons `char.isprintable()` and I am too lazy to
        //       implement it myself. And do I need it anyways?
        if c.is_ascii() {
            res.push(c);
            continue;
        }

        let mut buf = [0; 2];
        let hex = c.encode_utf16(&mut buf);
        let encoded = format!("\u{01}{:04X}", hex[0]);
        res.push_str(&encoded);
    }

    return Some(res);
}

pub fn decode_file(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let string = &line.unwrap();
        let decoded = decode_string(string).unwrap();
        res.push_str(&decoded);
        res.push_str("\r\n");
    }

    return Ok(res);
}

pub fn encode_file(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let string = &line.unwrap();
        let encoded = encode_string(string).unwrap();
        res.push_str(&encoded);
        res.push_str("\r\n");
    }

    return Ok(res);
}

pub fn decode(config: Config) -> Result<String, Box<dyn Error>> {
    let res = decode_file(&config.in_file_path)?;
    write_out_file(config, &res, "decoded")?;
    return Ok(res);
}

pub fn encode(config: Config) -> Result<String, Box<dyn Error>> {
    let res = encode_file(&config.in_file_path)?;
    write_out_file(config, &res, "encoded")?;
    return Ok(res);
}

pub fn write_out_file(config: Config, res: &str, action_str: &str) -> Result<(), Box<dyn Error>> {
    if config.tostd {
        println!("{res}");
        return Ok(());
    }

    let out_file_path = config.out_file_path.unwrap_or_else(|| {
        let mut a = config.in_file_path;
        a.set_extension("");
        let mut m = a.into_os_string();
        m.push("_");
        m.push(action_str);
        m.push(".txt");
        return m.into();
    });

    let mut outfile = File::create(&out_file_path).unwrap_or_else(|err| {
        println!("{res}");
        eprintln!("Error: can not write output into '{}': {}", out_file_path.display(), err);
        eprintln!("Result outputed to stdout instead"); // Not sure if this is good idea...
        std::process::exit(1);
    });

    let _ = write!(outfile, "{}", res);
    return Ok(());
}
