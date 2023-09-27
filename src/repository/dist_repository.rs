use crate::net::RemoteFile;

/// The name of the root branch in the dalamud-distrib repository.
///
/// This name corresponds to the version of Dalamud that is stored at the root of the repository instead of in a branch folder.
const PRIMARY_BRANCH_NAME: &str = "latest";

/// The URL of the repository where Dalamud releases are hosted.
const REPOSITORY_BASE_URL: &str = "https://goatcorp.github.io/dalamud-distrib";

/// The default filename that contains Dalamud and its dependencies.
const RELEASE_ARCHIVE_FILE_NAME: &str = "latest.zip";

/// The default filename that contains the version information for the Dalamud release.
const RELEASE_VERSION_INFO_FILE_NAME: &str = "version";

/// A struct that represents a Dalamud release repository / host.
#[derive(Debug)]
pub struct DistRepository {
    /// The URL to download releases from.
    pub url: String,
    /// The name of the file that contains Dalamud and its dependencies.
    pub release_file: String,
    /// The name of the file that contains the version information for the Dalamud release.
    #[allow(dead_code)]
    pub version_info_file: String,
}

impl Default for DistRepository {
    fn default() -> Self {
        Self::from_url_with_defaults(REPOSITORY_BASE_URL.to_string())
    }
}

impl DistRepository {
    pub fn from_url_with_defaults(url: String) -> Self {
        Self {
            url,
            release_file: RELEASE_ARCHIVE_FILE_NAME.to_string(),
            version_info_file: RELEASE_VERSION_INFO_FILE_NAME.to_string(),
        }
    }

    /// Returns the URL to the Dalamud release file for the given branch.
    pub fn get_download_url(&self, branch: &str) -> RemoteFile {
        if branch == PRIMARY_BRANCH_NAME {
            RemoteFile::from_url(format!("{}/{}", self.url, self.release_file))
        } else {
            RemoteFile::from_url(format!("{}/{}/{}", self.url, branch, self.release_file))
        }
    }

    /// Returns the URL to the Dalamud version info file for the given branch.
    pub fn get_version_info_url(&self, branch: &str) -> RemoteFile {
        if branch == PRIMARY_BRANCH_NAME {
            RemoteFile::from_url(format!("{}/{}", self.url, self.version_info_file))
        } else {
            RemoteFile::from_url(format!(
                "{}/{}/{}",
                self.url, branch, self.version_info_file
            ))
        }
    }
}
