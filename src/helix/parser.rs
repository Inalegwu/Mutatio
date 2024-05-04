use crate::cmd::cmd::ThemeOptions;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read};
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct HelixTheme {
    colors: HashMap<String, String>,
    scopes: HashMap<String, String>,
}

pub struct HelixThemeParser {
    theme: Option<HelixTheme>,
}

impl HelixThemeParser {
    pub fn new() -> Self {
        HelixThemeParser { theme: None }
    }

    pub fn execute(&mut self, file: &mut File, to: ThemeOptions) {
        let mut file_buf: String = String::new();
        let _ = file.read_to_string(&mut file_buf);
        let theme: HelixTheme = toml::from_str(&file_buf).expect("failed to parse toml file");

        match to {
            ThemeOptions::Helix => {
                println!("Source file is already a Helix Theme File")
            }
            ThemeOptions::VSCode => {
                println!("{:#?}", theme)
            }
        }
    }
}
