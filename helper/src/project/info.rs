use git2::Repository;
use serde::Deserialize;
use serde::Serialize;
use tokio;
#[derive(Serialize)]
pub struct Collaborator {
    pub url: String,
    pub img_url: String,
}
pub struct ProjectInfo {
    pub name: String,
    pub url: String,
    pub description: String,
    pub version: String,
    pub collaborator: Vec<Collaborator>,
    pub img: String,
}

impl ProjectInfo {
    pub fn from_repo(path: &str) -> Self {
        let repo = Repository::open(path).unwrap();
        let remote = repo.find_remote("origin").unwrap();
        let clear_url = remote.url().unwrap().trim_end_matches(".git");
        let parts: Vec<&str> = clear_url.split("/").collect();
        let owner = parts.get(parts.len() - 2).unwrap_or(&"Unknown");
        let name = parts.last().unwrap_or(&"Unknown").to_string();

        let api_url = format!("https://api.github.com/repos/{}/{}", owner, name);
        let version = repo
            .tag_names(None)
            .unwrap()
            .iter()
            .filter_map(|t| t)
            .last()
            .unwrap_or("0.1")
            .to_string();

        ProjectInfo {
            name: name,
            url: remote.url().unwrap_or("").to_string(),
            description: "".to_string(),
            version: version,
            collaborator: vec![
                Collaborator {
                    url: "https://github.com/josedddd1234".to_string(),
                    img_url: "https://avatars.githubusercontent.com/u/70914088?v=4".to_string(),
                },
                Collaborator {
                    url: "https://github.com/juancamilo0509".to_string(),
                    img_url: "https://avatars.githubusercontent.com/u/70924088?v=4".to_string(),
                },
            ],
            img: "../projectFilesSample/docs/assets/logo.png".to_string(),
        }
    }
}
