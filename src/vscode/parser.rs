use crate::cmd::cmd::ThemeOptions;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Error,
    fs::{File, Metadata},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenColors {
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

pub struct VSThemeParser {
    theme: Option<VsCodeTheme>,
}

impl VSThemeParser {
    pub fn new() -> Self {
        return VSThemeParser { theme: None };
    }

    pub fn execute(&mut self, file: File, to: ThemeOptions) -> Result<(), Error> {
        let theme: VsCodeTheme =
            serde_json::from_reader(file).expect("Error parsing vs code theme");

        self.theme = Some(theme);

        match to {
            ThemeOptions::Helix => {
                self.flatten_theme();
                println!("{:#?}", self.theme);
            }
            ThemeOptions::VSCode => {
                println!("File is already vscode theme file");
            }
            ThemeOptions::Nvim => {
                println!("[WIP]")
            }
        }

        Ok(())
    }

    pub fn flatten_theme(&mut self) {}
}
