use shared::models::{action::ConnectorType, color::RgbColor};
use strum::{EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, IntoStaticStr)]
pub enum ThemeColor {
    White,
    Pink,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Blank,
}

impl ThemeColor {
    pub fn to_fill_class(&self) -> String {
        let name: &'static str = self.into();
        format!("fill-{}", name.to_lowercase())
    }
    pub fn to_bg_class(&self) -> String {
        let name: &'static str = self.into();
        format!("bg-{}", name.to_lowercase())
    }
}

pub fn get_color_from_namespace(namespace: &str) -> ThemeColor {
    match namespace {
        "string" => ThemeColor::Purple,
        "boolean" => ThemeColor::Red,
        "integer" => ThemeColor::Blue,
        "float" => ThemeColor::Green,
        "os" => ThemeColor::Yellow,
        "resource" => ThemeColor::Orange,
        _ => ThemeColor::White,
    }
}

pub fn get_color_from_type(ty: ConnectorType) -> ThemeColor {
    match ty {
        ConnectorType::String => ThemeColor::Purple,
        ConnectorType::Boolean => ThemeColor::Red,
        ConnectorType::Integer => ThemeColor::Blue,
        ConnectorType::Float => ThemeColor::Green,
        ConnectorType::Flow => ThemeColor::White,
    }
}
