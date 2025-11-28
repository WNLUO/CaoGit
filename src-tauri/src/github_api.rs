use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRelease {
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
    pub html_url: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubAsset {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub download_count: u64,
    pub browser_download_url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRunsResponse {
    pub total_count: u64,
    pub workflow_runs: Vec<WorkflowRun>,
}

pub struct GitHubClient {
    token: Option<String>,
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("GitManager/0.1.0")
            .build()
            .unwrap();

        Self { token, client }
    }

    /// 解析 GitHub 仓库 URL，返回 (owner, repo)
    pub fn parse_repo_url(url: &str) -> Result<(String, String)> {
        // 支持格式：
        // https://github.com/owner/repo.git
        // git@github.com:owner/repo.git
        // https://github.com/owner/repo

        let url = url.trim_end_matches(".git");

        if let Some(caps) = url.strip_prefix("https://github.com/") {
            let parts: Vec<&str> = caps.split('/').collect();
            if parts.len() >= 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }

        if let Some(caps) = url.strip_prefix("git@github.com:") {
            let parts: Vec<&str> = caps.split('/').collect();
            if parts.len() >= 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }

        anyhow::bail!("Invalid GitHub URL format: {}", url)
    }

    /// 获取所有 Releases
    pub async fn list_releases(&self, owner: &str, repo: &str) -> Result<Vec<GitHubRelease>> {
        let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);

        let mut req = self.client.get(&url);

        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let response = req
            .send()
            .await
            .context("Failed to fetch releases from GitHub")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, text);
        }

        let releases: Vec<GitHubRelease> = response.json().await?;
        Ok(releases)
    }

    /// 获取最近的 Workflow Runs
    pub async fn list_workflow_runs(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<WorkflowRun>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs?per_page=10",
            owner, repo
        );

        let mut req = self.client.get(&url);

        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let response = req
            .send()
            .await
            .context("Failed to fetch workflow runs from GitHub")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, text);
        }

        let runs_response: WorkflowRunsResponse = response.json().await?;
        Ok(runs_response.workflow_runs)
    }

    /// 触发 workflow（通过创建标签）
    pub async fn trigger_release(
        &self,
        owner: &str,
        repo: &str,
        tag_name: &str,
    ) -> Result<String> {
        // 这个方法只是返回构建的 Actions URL
        // 实际的标签创建由 Git 操作完成
        Ok(format!(
            "https://github.com/{}/{}/actions",
            owner, repo
        ))
    }

    /// 重新运行失败的 workflow
    pub async fn rerun_workflow(&self, owner: &str, repo: &str, run_id: u64) -> Result<()> {
        let token = self
            .token
            .as_ref()
            .context("GitHub token required to rerun workflows")?;

        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs/{}/rerun",
            owner, repo, run_id
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to rerun workflow")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to rerun workflow ({}): {}", status, text);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_url() {
        let cases = vec![
            (
                "https://github.com/WNLUO/CaoGit.git",
                ("WNLUO", "CaoGit"),
            ),
            ("https://github.com/WNLUO/CaoGit", ("WNLUO", "CaoGit")),
            ("git@github.com:WNLUO/CaoGit.git", ("WNLUO", "CaoGit")),
            ("git@github.com:WNLUO/CaoGit", ("WNLUO", "CaoGit")),
        ];

        for (url, (expected_owner, expected_repo)) in cases {
            let (owner, repo) = GitHubClient::parse_repo_url(url).unwrap();
            assert_eq!(owner, expected_owner);
            assert_eq!(repo, expected_repo);
        }
    }
}
