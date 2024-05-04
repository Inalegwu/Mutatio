use clap::Parser;
use mutatio::cmd::cmd::{Args, ThemeOptions};
use mutatio::helix::parser::HelixThemeParser;
use mutatio::vscode::parser::VSThemeParser;
use std::fmt::Error;
use std::{fs::File, path::Path};

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let path = Path::new(&args.file);

 
    let mut file = File::open(path).expect("Couldn't open file");

    match args.from {
        ThemeOptions::VSCode => {
            let mut theme_parser = VSThemeParser::new();
            let _ = theme_parser.execute(file, args.to);
        }
        ThemeOptions::Helix => {
            let mut theme_parser = HelixThemeParser::new();
            // passing the path instead,'cause toml package is wierd
            let _ = theme_parser.execute(&mut file, args.to);
        }
        ThemeOptions::Nvim=>{
            println!("[WIP]")
        }
    }

    Ok(())
}
