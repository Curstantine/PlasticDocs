use crate::constants;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Default)]
pub struct ExternalLinkIcon {
    pub value: String,
    /// If raw_svg, value should be a valid xml that should be inside <svg> tag.
    pub raw_svg: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ExternalLink {
    pub name: String,
    pub url: String,
    pub icon: Option<ExternalLinkIcon>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Colors {
    pub bg_1: String,
    pub bg_2: String,
    pub bg_3: String,
    pub bg_accent: String,

    pub text_1: String,
    pub text_2: String,
    pub text_3: String,

    pub text_accent_1: String,
    pub text_accent_2: String,

    pub border: String,
    pub border_accent: String,
}

#[derive(Serialize, Deserialize)]
pub struct Font {
    pub font_name: String,
    /// Could be a cdn url or a path relative from /static
    pub url: String,
    pub format: String,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            font_name: String::from("Nunito Sans"),
            url: String::from("/nunito-sans.tff"),
            format: String::from("ttf"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decorations {
    pub font_family: Option<Font>,
    pub colors_dark: Colors,
    pub colors_light: Colors,
}

impl Default for Decorations {
    fn default() -> Self {
        Self {
            font_family: Some(Font::default()),
            colors_dark: Colors {
                bg_1: "rgb(23, 24, 25)".to_string(),
                bg_2: "rgb(26, 28, 29)".to_string(),
                bg_3: "rgb(38, 41, 42)".to_string(),
                bg_accent: "rgb(204, 61, 18)".to_string(),
                text_1: "rgb(232, 230, 227)".to_string(),
                text_2: "rgb(190, 185, 176)".to_string(),
                text_3: "rgb(105, 102, 97)".to_string(),
                text_accent_1: "rgb(255, 218, 206)".to_string(),
                text_accent_2: "rgba(255, 244, 240, 0.808)".to_string(),
                border: "rgb(44, 44, 44)".to_string(),
                border_accent: "rgb(204, 61, 18)".to_string(),
            },
            colors_light: Colors {
                bg_1: "rgb(255, 255, 255)".to_string(),
                bg_2: "rgb(248, 249, 252)".to_string(),
                bg_3: "rgb(237, 239, 244)".to_string(),
                bg_accent: "rgb(252, 220, 209)".to_string(),
                text_1: "rgb(47, 51, 72)".to_string(),
                text_2: "rgb(83, 87, 107)".to_string(),
                text_3: "rgb(113, 127, 154)".to_string(),
                text_accent_1: "rgb(255, 62, 0)".to_string(),
                text_accent_2: "rgba(255, 64, 0, 0.829)".to_string(),
                border: "rgb(237, 239, 244)".to_string(),
                border_accent: "rgb(255, 62, 0)".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub product_name: String,
    pub doc_version: String,
    /// Relative path from /static
    pub product_icon: String,
    pub external_links: Option<Vec<ExternalLink>>,
    pub decorations: Option<Decorations>,
}

pub fn read_config(path: Option<&Path>) -> Config {
    fn check(path: &Path) {
        if let Err(error) = fs::read(&path) {
            panic!("Failed to read the config file!! {error}");
        }
    }

    let path = path.unwrap_or_else(|| Path::new(constants::DEFAULT_CONFIG_PATH));
    check(path);

    let config: Config = confy::load_path(path).unwrap();

    config
}
