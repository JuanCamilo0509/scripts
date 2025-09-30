use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Requirements {
    #[serde(rename = "requirement")]
   pub  requirements: Vec<Requirement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Requirement {
   pub  id: String,
   pub  title: String,
   pub  description: String,
}
impl Requirements {
    pub fn read_requiremest(path: &str) -> Self {
        let toml_str = fs::read_to_string(path).expect("Failed to read the file");
        let content: Requirements = toml::from_str(&toml_str).unwrap();
        return content
    }   
}
