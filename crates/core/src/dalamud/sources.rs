use crate::net::RemoteResource;

/// An implementation that enables locating Dalamud release assets.
pub trait ReleaseSource {
    fn get_release_archive_file(&self, branch: &str) -> RemoteResource;
    fn get_version_info_file(&self, branch: &str) -> RemoteResource;
}

/// The name of the branch at the root of the repository.
///
/// This name corresponds to the version of Dalamud that is stored at the root of the repository (e.g. /version.zip).
///
/// **Note: This does not represent the git branch name**.
const OFFICIAL_DOWNLOAD_BASE_URL: &str = "https://goatcorp.github.io/dalamud-distrib";
const OFFICIAL_ROOT_BRANCH_NAME: &str = "latest";
const OFFICIAL_RELEASE_ARCHIVE_FILENAME: &str = "latest.zip";
const OFFICIAL_VERSION_INFO_FILENAME: &str = "version";

/// A [`ReleaseSource`] implementation for the official dalamud-distrib repository at `goatcorp/dalamud-distrib`.
#[derive(Debug, Clone, Copy)]
pub struct GoatcorpReleaseSource;

impl Default for GoatcorpReleaseSource {
    fn default() -> Self {
        GoatcorpReleaseSource
    }
}

impl ReleaseSource for GoatcorpReleaseSource {
    /// Get the [`RemoteResource`] for the given Dalamud branch release archive.
    fn get_release_archive_file(&self, branch: &str) -> RemoteResource {
        if branch == OFFICIAL_ROOT_BRANCH_NAME {
            RemoteResource::from_url(format!(
                "{OFFICIAL_DOWNLOAD_BASE_URL}/{OFFICIAL_RELEASE_ARCHIVE_FILENAME}",
            ))
        } else {
            RemoteResource::from_url(format!(
                "{OFFICIAL_DOWNLOAD_BASE_URL}/{branch}/{OFFICIAL_RELEASE_ARCHIVE_FILENAME}"
            ))
        }
    }

    /// Get the [`RemoteResource`] for the given Dalamud branch version info file.
    fn get_version_info_file(&self, branch: &str) -> RemoteResource {
        if branch == OFFICIAL_ROOT_BRANCH_NAME {
            RemoteResource::from_url(format!(
                "{OFFICIAL_DOWNLOAD_BASE_URL}/{OFFICIAL_VERSION_INFO_FILENAME}"
            ))
        } else {
            RemoteResource::from_url(format!(
                "{OFFICIAL_DOWNLOAD_BASE_URL}/{branch}/{OFFICIAL_VERSION_INFO_FILENAME}"
            ))
        }
    }
}
