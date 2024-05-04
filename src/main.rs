use clap::Parser;
use mutatio::cmd::cmd::{Args, ThemeOptions};
use mutatio::vscode::parser::VSThemeParser;
use std::fmt::Error;
use std::{fs::File, path::Path};

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let path = Path::new(&args.file);

    let file = File::open(path).expect("Couldn't open file");

    match args.from {
        ThemeOptions::VSCode => {
            let mut theme_parser = VSThemeParser::new();
            let _ = theme_parser.execute(file, args.to);
        }
        ThemeOptions::Helix => {
            println!("[WIP]")
        }
    }

    Ok(())
}
