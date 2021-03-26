use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {

    #[serde(default = "default_font_id")]
    pub font_id: usize,

    pub lines: Vec<String>,

    #[serde(default)]
    pub color: TextColor,

    #[serde(default)]
    pub wait: Option<f32>,

}

const fn default_font_id() -> usize {
    1
}

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
pub enum TextColor {

    White,
    Gray,
    Black,
    Red,
    Blue,

}

impl Default for TextColor {
    fn default() -> Self {
        Self::Black
    }
}

impl Message {

    pub fn new(lines: Vec<String>, color: TextColor, wait: Option<f32>) -> Self {
        Self {
            font_id: default_font_id(),
            lines,
            wait,
            color,
        }
    }

}