use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Steps {
    #[serde(rename = "step")]
   pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
   pub title: String,
   pub description: String,
   pub image: String,
}

impl Steps {
    pub fn read_steps(path: &str) -> Steps {
        let toml_str = fs::read_to_string(path).expect("Failed to read the file");
        let content: Steps = toml::from_str(&toml_str).unwrap();
        return content
    }
}
