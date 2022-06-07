use crate::config::{Colors, Config, Font};
use std::fs;

pub fn make_configuration(conf: Config) {
    let conf_str = format!(
        "export const DOC_VERSION = \"{version}\"; \
         export const PRODUCT_NAME = \"{name}\"; \
         export const PRODUCT_ICON = \"{icon}\"; \
         export const EXTERNAL_LINKS = []; \
         export const PAGES = []; \
        ",
        version = conf.doc_version,
        name = conf.product_name,
        icon = conf.product_icon
    );

    let decor = conf.decorations.unwrap_or_default();
    let font = decor.font_family.unwrap_or_default();

    let style_conf = format!(
        "{font_patch} \
         .light {{ {light_colors} }} \
         .dark {{ {dark_colors} }} \
        ",
        font_patch = make_css_font(&font),
        light_colors = make_css_styles(decor.colors_light, &font),
        dark_colors = make_css_styles(decor.colors_dark, &font)
    );
}

pub fn make_css_font(font: &Font) -> String {
    format!(
        "@font-face {{ \
            font-family: \"{name}\"; \
            src: url(\"{src}\") format({format}) \
         }}
        ",
        name = font.font_name,
        src = font.url,
        format = font.format
    )
}

pub fn make_css_styles(colors: Colors, font: &Font) -> String {
    format!(
        "--bg-1: {bg_1}; \
         --bg-2: {bg_2}; \
         --bg-3: {bg_3}; \
         --bg-accent: {bg_accent}; \
         --font-family: {fonts} \
         --text-dark: {text_1}; \
         --text-light: {text_2}; \
         --text-lighter: {text_3} \
         --text-accent: {text_accent_1}; \
         --text-accent-lighter: {text_accent_2}; \
         --border: {border}; \
         --border-accent: {border_accent}; \
        ",
        bg_1 = colors.bg_1,
        bg_2 = colors.bg_2,
        bg_3 = colors.bg_3,
        bg_accent = colors.bg_accent,
        fonts = format_args!("\"{}\", sans-serif", font.font_name),
        text_1 = colors.text_1,
        text_2 = colors.text_2,
        text_3 = colors.text_3,
        text_accent_1 = colors.text_accent_1,
        text_accent_2 = colors.text_accent_2,
        border = colors.border,
        border_accent = colors.border_accent
    )
}
