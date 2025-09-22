use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Decisions {
    #[serde(rename = "decision")]
    decisions: Vec<Decision>,
}

#[derive(Deserialize, Debug)]
struct Decision {
    id: String,
    title: String,
    rationale: String,
    related_requirements: Vec<String>,
    related_tests: Vec<String>,
    status: String,
}

#[derive(Deserialize, Debug)]
struct Requirements {
    #[serde(rename = "requirement")]
    requirements: Vec<Requirement>,
}

#[derive(Deserialize, Debug)]
struct Requirement {
    id: String,
    title: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Tests {
    #[serde(rename = "test")]
    tests: Vec<Test>,
}
#[derive(Deserialize, Debug)]
struct Test {
    id: String,
    requirement_id: String,
    description: String,
    expected: String,
    result: String,
    status: String,
    evidence: String,
}

fn read_tests(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path).expect("Failed to read the file");
    let content: Tests = toml::from_str(&toml_str)?;
    for test in content.tests {
        println!("{:?}", test.id);
        println!("{:?}", test.requirement_id);
        println!("{:?} \n", test.description);
        println!("{:?}", test.expected);
        println!("{:?}", test.result);
        println!("{:?}", test.status);
        println!("{:?} \n", test.evidence);
    }
    Ok(())
}

fn read_design(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path).expect("Failed to read the file");
    let content: Decisions = toml::from_str(&toml_str)?;
    for design in content.decisions {
        println!("{:?}", design.id);
        println!("{:?}", design.title);
        println!("{:?}\n", design.rationale);
        println!("{:?}", design.related_requirements);
        println!("{:?}", design.related_tests);
        println!("{:?}", design.status);
    }
    Ok(())
}

fn read_requiremest(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path).expect("Failed to read the file");
    let content: Requirements = toml::from_str(&toml_str)?;
    for req in content.requirements {
        println!("{:?}", req.id);
        println!("{:?}", req.title);
        println!("{:?}\n", req.description);
    }
    Ok(())
}

fn main() {
    let reqpath = "./projectFilesSample/docs/requirements.toml";
    let testspath = "./projectFilesSample/docs/testing.toml";
    let designpath = "./projectFilesSample/docs/design-decisions.toml";
    read_requiremest(reqpath.to_string()).expect("Something went wrong parsing req");
    read_tests(testspath.to_string()).expect("Something went wrong parsing tests");
    read_design(designpath.to_string()).expect("Something went wrong parsing design decisions");
}
