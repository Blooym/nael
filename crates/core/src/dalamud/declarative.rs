use crate::net::RemoteResource;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, fs::read_to_string, path::Path, str::FromStr};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct DeclarativeConfig {
    tracks: HashMap<String, ReleaseTrack>,
}

impl FromStr for DeclarativeConfig {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl DeclarativeConfig {
    /// Get the file at the given path and returns a [`DeclarativeConfig`] from it.
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

    /// Get the version info from the [`RemoteFile`] and parse it into [`DalamudVersionInfo`].
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ReleaseTrack {
    key: Option<String>,
    applicable_game_version: String,
    runtime_version: String,
}
