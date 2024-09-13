use std::ops::Not;

use anyhow::bail;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;

use crate::os::OperatingSystem;

pub struct RepologyClient {
    client: reqwest::Client,
    base_domain: &'static str,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub repo: String,
    pub srcname: Option<String>,
    pub visiblename: String,
    pub version: String,
    pub origversion: Option<String>,
    pub status: String,
    pub vulnerable: Option<bool>,
    #[serde(default)]
    pub licenses: Vec<String>,
    pub summary: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub subrepo: Option<String>,
    pub binname: Option<String>,
    #[serde(default)]
    pub maintainers: Vec<String>,
}

impl RepologyClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_domain: "https://repology.org/api",
        }
    }

    pub async fn get_projects_for_os(
        &self,
        project_name: &str,
        os: OperatingSystem,
    ) -> anyhow::Result<Vec<Project>> {
        let package_managers = os.package_managers();

        let url = format!(
            "{base_domain}/v1/project/{project_name}",
            base_domain = self.base_domain
        );
        let url = Url::parse(&url)?;

        let resp = self
            .client
            .get(url)
            .header(USER_AGENT, "vrmiguel")
            .send()
            .await?;

        if resp.status().is_success().not() {
            let status_code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();

            bail!("Response failed: [{status_code}] {text}")
        }

        // All projects returned
        let mut projects: Vec<Project> = resp.json().await?;
        projects.retain(|project| {
            package_managers.iter().any(|package_manager| {
                package_manager
                    .repology_repository_prefix()
                    .iter()
                    .any(|repo_prefix| project.repo.starts_with(repo_prefix))
            })
        });

        Ok(projects)
    }

    pub async fn get_projects(&self, project_name: &str) -> anyhow::Result<Vec<Project>> {
        let Some(current_os) = OperatingSystem::detect() else {
            bail!("Current operating system or Linux distro unsupported")
        };

        self.get_projects_for_os(project_name, current_os).await
    }
}
