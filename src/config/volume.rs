use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub icons: Vec<String>,
    pub show_text: bool,
    pub enabled: bool,
    pub delay: f64,
    pub card: String,
}

impl Default for Volume {
    fn default() -> Self {
        Volume {
            icons: vec![String::from("")],
            show_text: false,
            enabled: false,
            delay: 0.17,
            card: String::from("ALSA"),
        }
    }
}
