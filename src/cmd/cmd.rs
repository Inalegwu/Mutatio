use clap::Parser;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ThemeOptions {
    VSCode,
    Helix,
    Nvim
}

#[derive(Parser, Debug)]
#[command(version, about,long_about=None)]
pub struct Args {
    pub from: ThemeOptions,

    pub file: String,

    pub to: ThemeOptions,
}
