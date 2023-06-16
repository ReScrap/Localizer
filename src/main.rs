use std::path::PathBuf;
use structopt::{StructOpt, paw};

#[derive(StructOpt)]
struct CommandArgs {
    /// File to process
    #[structopt(name = "file", parse(from_os_str))]
    file: PathBuf,

    /// If enabled, this program will outptut result to stdout instead of file
    #[structopt(short, long)]
    tostd: bool,
}

#[derive(StructOpt)]
#[structopt(about = "Tool for decoding and encoding Scrapland localization files")]
enum Command {
    /// Decode encoded file
    Decode {
        #[structopt(flatten)]
        args: CommandArgs,

        // Note: outfile is extracted from CommandArgs because of help text
        /// Output file. If not specified, outptut will be in file "<input>_decoded.txt"
        #[structopt(short, long)]
        outfile: Option<PathBuf>,

    },
    /// Encode decoded file
    Encode {
        #[structopt(flatten)]
        args: CommandArgs,

        // Note: outfile is extracted from CommandArgs because of help text
        /// Output file. If not specified, outptut will be in file "<input>_encdoded.txt"
        #[structopt(short, long)]
        outfile: Option<PathBuf>,
    },
}

#[paw::main]
fn main(args: Command) {
    match args {
        Command::Decode { args, outfile } => {
            let input_path_str = args.file.to_owned();

            let config = rescrap_localizer::Config  {
                in_file_path: args.file,
                out_file_path: outfile,
                tostd: args.tostd
            };

            rescrap_localizer::decode(config).unwrap_or_else(|err| {
                eprintln!("Error: can not decode file '{}': {}", input_path_str.display(), err);
                std::process::exit(1);
            });
        },
        Command::Encode { args, outfile } => {
            let input_path_str = args.file.to_owned();

            let config = rescrap_localizer::Config {
                in_file_path: args.file,
                out_file_path: outfile,
                tostd: args.tostd
            };

            rescrap_localizer::encode(config).unwrap_or_else(|err| {
                eprintln!("Error: can not encode file '{}': {}", input_path_str.display(), err);
                std::process::exit(1);
            });
        }
    }
}
