use clap::Parser;
use mutatio::vscode::parser::VSThemeParser;
use std::fmt::Error;
use std::{fs::File, path::Path};

#[derive(Clone, Debug, clap::ValueEnum)]
enum ThemeOptions {
    VSCode,
    Helix,
}

#[derive(Parser, Debug)]
#[command(version, about,long_about=None)]
struct Args {
    from: ThemeOptions,

    file: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let path = Path::new(&args.file);

    let file = File::open(path).expect("Couldn't open file");

    match args.from {
        ThemeOptions::VSCode => {
            let mut theme_parser = VSThemeParser::new();
            let _ = theme_parser.execute(file);
        }
        ThemeOptions::Helix => {
            println!("[WIP]")
        }
    }

    Ok(())
}
