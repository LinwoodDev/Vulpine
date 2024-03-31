use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor(u8, u8, u8);

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor(r, g, b)
    }
    pub fn from_value(value: u32) -> RgbColor {
        let r = (value >> 16) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        RgbColor(r, g, b)
    }
    pub fn from_hex(hex : String) -> RgbColor {
        let hex = hex.trim_start_matches("#");
        let mut r = 0_u8;
        let mut g = 0_u8;
        let mut b = 0_u8;
        match hex.len() {
            6 | 8 => {
                r = u8::from_str_radix(&hex[0..2], 16).unwrap();
                g = u8::from_str_radix(&hex[2..4], 16).unwrap();
                b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            }
            3 | 4 => {
                r = u8::from_str_radix(&hex[0..1], 16).unwrap();
                g = u8::from_str_radix(&hex[1..2], 16).unwrap();
                b = u8::from_str_radix(&hex[2..3], 16).unwrap();
            }
            _ => {}
        }
        RgbColor(r, g, b)
    }
    pub fn to_value(&self) -> u32 {
        let r = self.0 as u32;
        let g = self.1 as u32;
        let b = self.2 as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}
