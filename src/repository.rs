/// The URL of the repository where Dalamud releases are hosted.
const DEFAULT_DISTRIB_REPO_URL: &str = "https://github.com/goatcorp/dalamud-distrib/raw/main";

/// The name of the file that contains the Dalamud release within the repository/branch folder.
pub const DEFAULT_DISTRIB_REPO_FILE: &str = "latest.zip";

#[derive(Debug)]
pub struct Repository {
    pub url: String,
    release_file: String,
}

impl Default for Repository {
    fn default() -> Self {
        Self::new(
            DEFAULT_DISTRIB_REPO_URL.to_string(),
            DEFAULT_DISTRIB_REPO_FILE.to_string(),
        )
    }
}

impl Repository {
    /// Creates a new repository with the given URL and Dalamud release file name.
    pub fn new(url: String, release_file: String) -> Self {
        Self { url, release_file }
    }

    /// Returns the URL to the Dalamud release file for the given branch.
    pub fn get_download_url(&self, branch: &str) -> String {
        if branch == "latest" {
            format!("{}/{}", self.url, self.release_file)
        } else {
            format!("{}/{}/{}", self.url, branch, self.release_file)
        }
    }
}
