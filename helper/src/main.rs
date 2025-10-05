mod project;
use clap::Parser;
use project::{info::ProjectInfo, requirements::Requirements, steps::Steps};
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let path = args.path;

    let reqpath = path
        .join("docs/requirements.toml")
        .to_str()
        .unwrap()
        .to_string();
    let stepspath = path.join("docs/steps.toml").to_str().unwrap().to_string();

    let requirements = Requirements::read_requiremest(&reqpath);
    let project_info = ProjectInfo::from_repo(&path.to_str().unwrap().to_string());
    let steps = Steps::read_steps(&stepspath);

    let tera = Tera::new("template/**/*.html").expect("Tera error");
    let mut context = Context::new();
    context.insert("project_logo", &project_info.img);
    context.insert("project_url", &project_info.url);
    context.insert("project_name", &project_info.name);
    context.insert("project_description", &project_info.github_info.description);
    context.insert("project_version", &project_info.version);
    context.insert("requirements", &requirements.requirements);
    context.insert("steps", &steps.steps);
    context.insert("n", &steps.steps.iter().count());

    let rendered = tera.render("main.html", &context).unwrap();
    fs::write(path.join("docs/manual.html"), rendered).expect("Can't find docs")
}
