use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct Decisions {
    #[serde(rename = "decision")]
    pub _decisions: Vec<Decision>,
}

#[derive(Deserialize, Debug)]
pub struct Decision {
   pub  _id: String,
   pub _title: String,
   pub _rationale: String,
   pub _related_requirements: Vec<String>,
   pub _related_tests: Vec<String>,
   pub _status: String,
}
