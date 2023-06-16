use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::string::String;
use structopt::{StructOpt, paw};

#[derive(StructOpt)]
enum Command {
    Decode,
    Encode,
}

#[derive(StructOpt)]
#[structopt(about = "Tool for decoding/encoding Scrapland localisation files")]
struct Args {
    /// File to process
    #[structopt(name = "file", parse(from_os_str))]
    file: PathBuf,

    #[structopt(subcommand)]
    cmd: Command,

    /// Output file. If not specified, outptut will be in file "<input>_<decoded/encdoded>.txt"
    #[structopt(name = "outptut", parse(from_os_str))]
    outptut: Option<PathBuf>,

    /// If enabled, this program will outptut result to stdout instead of file
    #[structopt(short, long)]
    tostd: bool,
}


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

fn decode_file(path: &PathBuf) -> Result<String, std::io::Error> {
    let mut res = String::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let string = &line.unwrap();
        let decoded = decode_string(string).unwrap();
        res.push_str(&decoded);
        res.push('\n');
    }

    return Ok(res);
}

fn encode_file(_path: &PathBuf) -> Result<String, std::io::Error> {
    unimplemented!();
}

#[paw::main]
fn main(args: Args) -> std::io::Result<()> {
    let res;
    let outfile_cmd;

    match args.cmd {
        Command::Decode => {
            res = decode_file(&args.file)?;
            outfile_cmd = "_decoded.txt";
        },
        Command::Encode => {
           res = encode_file(&args.file)?;
            outfile_cmd = "_encoded.txt";
        }
    }

    if args.tostd {
        println!("{res}");
        return Ok(());
    }

    let mut outputfile;

    match args.outptut {
        Some(output) => outputfile = output,
        None => {
            outputfile = args.file;
            outputfile.set_extension("");
            let mut m = outputfile.into_os_string();
            m.push(outfile_cmd);
            outputfile = m.into()
        },
    }

    let mut outfile = File::create(outputfile)?;
    let _ = write!(outfile, "{}", res);

    return Ok(());
}
