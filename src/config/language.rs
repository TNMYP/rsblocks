use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub enabled: bool,
    pub delay: f64,
}

impl Default for Language {
    fn default() -> Self {
        Language{ 
            enabled: true,
            delay: 30.0,
        }
    }
}
