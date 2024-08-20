#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "subType")]
    pub subtype: Option<String>,
    pub lastLine: Option<u32>,
    pub lastColumn: Option<u32>,
    pub firstLine: Option<u32>,
    pub firstColumn: Option<u32>,
    pub message: String,
    pub extract: Option<String>,
    pub hiliteStart: Option<u32>,
    pub hiliteLength: Option<u32>,
}
