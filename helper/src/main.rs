mod project;
use std::fs;
use project::{steps::Steps, info::ProjectInfo, requirements::Requirements};
use tera::{Context, Tera};

fn main() {
    // TODO: Implement clap or just arguments to call the main project folder.
    // TODO: Finish the code to get from the GitHub end point.
    // TODO: Enhancement the pdf export from Firefox and chrome.

    let reqpath = "./projectFilesSample/docs/requirements.toml";
    let stepspath = "./projectFilesSample/docs/steps.toml";

    let requirements = Requirements::read_requiremest(reqpath);
    let project_info = ProjectInfo::from_repo("./projectFilesSample/");
    let steps = Steps::read_steps(stepspath);

    let tera = Tera::new("template/**/*.html").expect("Tera error");
    let mut context = Context::new();
    context.insert("project_name", &project_info.name);
    context.insert("project_description", &project_info.description);
    context.insert("project_version", &project_info.version);
    context.insert("project_logo", &project_info.img);
    context.insert("project_url", &project_info.url);
    context.insert("collaborators", &project_info.collaborator);
    context.insert("requirements", &requirements.requirements);
    context.insert("steps", &steps.steps);
    context.insert("n", &steps.steps.iter().count());

    let rendered = tera.render("main.html", &context).unwrap();
    fs::write("template/output.html", rendered).expect("Something went wrong")
}
