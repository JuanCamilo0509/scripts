use std::fmt::format;

use git2::Repository;
use reqwest::Client;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde::Serialize;
use tokio;

#[derive(Serialize, Deserialize)]
pub struct GithubInformation {
    pub description: String,
}

#[derive(Serialize)]
pub struct Collaborator {
    pub url: String,
    pub img_url: String,
}
pub struct ProjectInfo {
    pub name: String,
    pub url: String,
    pub version: String,
    pub github_info: GithubInformation,
    pub img: String,
}

impl GithubInformation {
    #[tokio::main]
    pub async fn api_call(
        owner: &str,
        rep_name: &str,
    ) -> Result<GithubInformation, Box<dyn std::error::Error>> {
        let api_url = format!("https://api.github.com/repos/{}/{}", owner, rep_name);
        let client = Client::new();
        let response = client
            .get(api_url)
            .header(USER_AGENT, "repo")
            .send()
            .await?;
        let data: GithubInformation = response.json().await?;
        if data.description.is_empty() {
            println!("Empty response")
        }
        Ok({
            GithubInformation {
                description: data.description.to_string(),
            }
        })
    }
}

impl ProjectInfo {
    pub fn from_repo(path: &str) -> Self {
        let repo = Repository::open(path).unwrap();
        let remote = repo.find_remote("origin").unwrap();
        let clear_url = remote.url().unwrap().trim_end_matches(".git");
        let parts: Vec<&str> = clear_url.split("/").collect();
        let owner = parts.get(parts.len() - 2).unwrap_or(&"Unknown");
        let name = parts.last().unwrap_or(&"Unknown").to_string();
        let github_info = GithubInformation::api_call(owner, &name).unwrap();

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
            version: version,
            github_info: github_info,
            img: format!("{}/{}", path, "docs/assets/logo.png"),
        }
    }
}
