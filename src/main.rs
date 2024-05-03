use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::File, path::Path};

#[derive(Parser, Debug)]
#[command(version, about,long_about=None)]
struct Args {
    file: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenColors {
    name:Option<String>,
    scope: Vec<String>,
    settings: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VsCodeTheme {
    name: String,
    r#type: Type,
    colors: HashMap<String, String>,
    token_colors: Vec<TokenColors>,
    semantic_highlighting: bool,
    semantic_token_colors: HashMap<String, String>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.file);

    let file = File::open(path);

    match file {
        Ok(v) => {
            let theme: VsCodeTheme =
                serde_json::from_reader(v).expect("Error parsing vs code theme");
            println!("{:#?}", theme)
        }
        Err(e) => {
            panic!("Error Occurred {}", e);
        }
    }
}
