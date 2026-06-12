use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InputItems(pub Vec<InputItem>);

impl InputItems {
    pub fn new(items: Vec<InputItem>) -> Self {
        Self(items)
    }

    pub fn print(&self) {
        println!(
            "{}",
            serde_json::to_string(self).expect("serde_json failed")
        );
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputItem {
    display: String,
    raw: serde_json::Value,
}

impl InputItem {
    pub fn new(display: String, raw: impl Into<serde_json::Value>) -> Self {
        Self {
            display,
            raw: raw.into(),
        }
    }
}

