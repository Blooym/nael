use crate::net::RemoteResource;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs::read_to_string, path::Path, str::FromStr};

// Note: https://kamori.goats.dev/Dalamud/Release/Meta exists and has caches of Dalamud releases and is how
// the official launchers perform updates.
// Right now this URL isn't being used, but it could be in the future to fetch releases in a better way, so
// it is being left as a note here.

/// Version information for a Dalamud release.
///
/// # Warning
/// The structure of version data may be changed by a upstream source at any time without warning
/// and break existing installations. Care should be put into handling cases like this where possible.
///
/// # Compatibility
/// This struct was built by manually looking at the `version` file on the official `goatcorp/dalamud-distrib` repository.
/// it may not work with 3rd party release sources.
///
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct DalamudVersionInfo {
    /// Assembly version string of Dalamud at release.
    pub assembly_version: String,
    /// Git commit hash of the Dalamud at release.
    pub git_sha: Option<String>,
    /// Revision number of Dalamud at release.
    pub revision: Option<String>,
}

impl FromStr for DalamudVersionInfo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl DalamudVersionInfo {
    /// Get the file at the given path and returns a [`DalamudVersionInfo`] from it.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When a failure occurs reading the file at the given path.
    /// * When serialization fails.
    pub fn from_path_ref<P: AsRef<Path> + Debug>(path: &P) -> Result<Self> {
        read_to_string(path)
            .with_context(|| format!("failed read file at {path:?}"))?
            .parse::<Self>()
            .with_context(|| format!("unable to deserialize file at {path:?}"))
    }

    /// Get the version info from the [`RemoteResource`] and parse it into [`DalamudVersionInfo`].
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any network failure occurs.
    /// * When serialization fails.
    pub async fn from_remote_file(file: &RemoteResource) -> Result<Self> {
        file.read_to_string()
            .await?
            .parse::<Self>()
            .with_context(|| format!("unable to deserialize resource at {}", file.url))
    }
}
