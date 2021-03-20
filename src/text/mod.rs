use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct Message {

    #[serde(default = "default_font_id")]
    pub font_id: usize,
    pub message: Vec<String>,
    #[serde(default)]
    pub color: TextColor,
    #[serde(default = "default_wait_for_input")]
    pub wait_for_input: bool,

}

impl Default for Message {
    fn default() -> Self {
        Self {
            font_id: default_font_id(),
            color: TextColor::default(),
            message: vec![String::from("Default message")],
            wait_for_input: default_wait_for_input(),
        }
    }
}

const fn default_font_id() -> usize {
    1
}

const fn default_wait_for_input() -> bool {
    true
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

    pub fn new(message: Vec<String>, wait_for_input: bool,) -> Self {
        Self::with_color(message, wait_for_input, TextColor::default())
    }

    pub fn with_color(message: Vec<String>, wait_for_input: bool, color: TextColor) -> Self {
        Self {
            message,
            wait_for_input,
            color,
            ..Default::default()
        }
    }

}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct MessageSet {
    pub messages: Vec<Message>,
}

impl MessageSet {

    pub fn new(font_id: usize, color: TextColor, messages: Vec<Vec<String>>) -> Self {
        let mut messages_vec = Vec::new();
        for message in messages {
            messages_vec.push(Message {
                font_id: font_id,
                message: message,
                color: color,
                wait_for_input: true,
            })
        }
        Self {
            messages: messages_vec,
        }
    }

    pub fn get_phrase(&self, index: usize) -> &Message {
        &self.messages[index]
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
    
}

impl Default for MessageSet {
    fn default() -> Self {
        Self {
            messages: vec![Message::default()],
        }
    }
}