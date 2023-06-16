# ReScrap Localizer

The Scrapland localisation files are located in the `Scrapland\Language` folder. You can easily mod them, but only if lines are written with ASCII characters.

The Problem starts when non-ASCII characters are used in your language. Characters are presented by their UTF-16BE (Big Endian) HEX value with a `0x01` byte at the start of every character.

To change these lines you need to decode them into human-readable format, then encode them back to the Scrapland format.

This is what this tool is for.

### HOW TO SETUP

```console
git clone https://github.com/ReScrap/Localizer
cd Localizer
cargo build --relese
```

### Help Screen

```console
$ ./target/release/rescrap_localizer help
rescrap_localizer 0.1.0
Tool for decoding and encoding Scrapland localization files

USAGE:
    rescrap_localizer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Decode encoded file
    encode    Encode decoded file
    help      Prints this message or the help of the given subcommand(s)

$ ./target/release/rescrap_localizer decode help
rescrap_localizer-decode 0.1.0
Decode encoded file

USAGE:
    rescrap_localizer decode [FLAGS] [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -t, --tostd      If enabled, this program will outptut result to stdout instead of file
    -V, --version    Prints version information

OPTIONS:
    -o, --outfile <outfile>    Output file. If not specified, outptut will be in file "<input>_decoded.txt"

ARGS:
    <file>    File to process
```


### HOW TO USE

In your command line window, after navigating into the folder with the program, use the following commands to decode & encode the language files:

* Decode:
```bash
$ ./target/release/rescrap_localizer decode <filename.txt>
```
This will output a human-readable version of the provided file with the name <filename_decoded.txt>


* Encode:
```bash
$ ./target/release/rescrap_localizer encode <filename.txt>
```
This will output a version of the provided file encoded for Scrapland with the name <filename_encoded.txt>

### HOW TO USE THE NEW TRANSLATION

Just replace the file `Scrapland\Language\<Your_language.txt>` and run the game.
