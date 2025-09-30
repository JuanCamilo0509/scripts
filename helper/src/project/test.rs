use serde::Deserialize;
use std::fs;
#[derive(Deserialize, Debug)]
pub struct Tests {
    #[serde(rename = "test")]
    _tests: Vec<Test>,
}
#[derive(Deserialize, Debug)]
pub struct Test {
    _id: String,
    _requirement_id: String,
    _description: String,
    _expected: String,
    _result: String,
    _status: String,
    _evidence: String,
}

impl Test {
pub fn read_tests(path: &str) -> Result<Tests, Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path).expect("Failed to read the file");
    let content: Tests = toml::from_str(&toml_str)?;
    Ok(content)
}
}
