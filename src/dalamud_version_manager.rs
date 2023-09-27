use crate::{fs::directories::get_versions_dir, logger::warning, repository::DistRepository};
use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{self, File},
    path,
    time::Duration,
};
use tempfile::tempdir;
use zip::ZipArchive;

/// The template for the progress bar used when extracting the Dalamud archive.
const EXTRACT_PROGRESS_BAR_TEMPLATE: &str = "[{elapsed_precise}] {spinner:.green} {msg}";

/// A struct that contains logic for interacting with Dalamud versions.
#[derive(Debug)]
pub struct DalamudVersionManager {}

impl DalamudVersionManager {
    /// Extracts the given file to the given directory.
    fn extract_file_with_progress(path: &path::PathBuf, extract_dir: &path::PathBuf) -> Result<()> {
        // Create the progress bar.
        let extract_progress_bar = ProgressBar::new_spinner();
        extract_progress_bar.set_style(
            ProgressStyle::default_spinner()
                .template(EXTRACT_PROGRESS_BAR_TEMPLATE)
                .unwrap()
                .progress_chars("#>-"),
        );
        extract_progress_bar.set_message("Extracting archive...");
        extract_progress_bar.enable_steady_tick(Duration::from_millis(100));

        // Extract the archive.
        let mut archive = ZipArchive::new(File::open(path)?)?;
        archive.extract(extract_dir)?;
        extract_progress_bar.finish_and_clear();

        Ok(())
    }

    /// Installs the given version of Dalamud to the versions directory.
    pub async fn create(branch: &str, replace_existing: bool) -> Result<()> {
        // Get relevant directories.
        let versions_dir = get_versions_dir().context("Failed to get versions directory.")?;
        let extract_dir = versions_dir.join(branch);

        // Check if the version is already installed and handle replace accordingly.
        if extract_dir.exists() {
            if replace_existing {
                fs::remove_dir_all(&extract_dir)?;
            } else {
                return Err(anyhow!(
                    "Dalamud version {} is already installed. Use --replace to replace it.",
                    branch,
                ));
            }
        }

        // Format the download URL and create a temporary directory to download to.
        let dir = tempdir()?;
        let archive_path: path::PathBuf = dir.path().join("download.zip");
        let archive_url = DistRepository::default().get_download_url(branch);

        // Download & extract the archive.
        archive_url
            .download_with_progress(archive_path.clone())
            .await?;
        Self::extract_file_with_progress(&archive_path, &extract_dir)?;

        let version_url = DistRepository::default().get_version_info_url(branch);
        if let Err(err) = version_url
            .download_with_progress(extract_dir.join("version"))
            .await
        {
            warning!("Unable to obtain version information: {}\nThis may cause issues when trying to use the update command later.", err);
        }

        // Remove the temporary directory explicitly.
        // (This is also done automatically when the dir goes out of scope but we want to do it explicitly incase it fails.)
        dir.close()?;

        Ok(())
    }

    /// Removes the given version of Dalamud from the versions directory.
    /// This will also unlink the version if it is the currently active version.
    pub fn remove(branch: &str) -> Result<()> {
        let versions_dir = get_versions_dir().context("Failed to get versions directory.")?;

        let branch_dir = versions_dir.join(branch);
        if !branch_dir.exists() {
            return Err(anyhow!("Unable to find {} in versions directory.", branch));
        }

        // Unset the current version if it is the one being removed.
        let current = Self::get_current()?;
        if let Some(current) = current {
            if current == branch {
                Self::unset_current()?;
            }
        }
        fs::remove_dir_all(&branch_dir)?;

        Ok(())
    }

    /// Returns whether the given version of Dalamud is installed.
    pub fn exists(branch: &str) -> Result<bool> {
        let versions_dir = get_versions_dir().context("Failed to get versions directory.")?;
        let branch_dir = versions_dir.join(branch);
        Ok(branch_dir.exists())
    }

    /// Returns a list of all installed versions of Dalamud.
    pub fn get_all_installed() -> Result<Vec<String>> {
        let versions_dir = get_versions_dir().context("Failed to get versions directory.")?;
        if !versions_dir.exists() {
            return Ok(vec![]);
        }

        let mut versions = fs::read_dir(versions_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let file_name = path.file_name()?.to_str()?.to_owned();
                Some(file_name)
            })
            .collect::<Vec<_>>();
        versions.sort();
        Ok(versions)
    }

    /// Returns the currently active version of Dalamud.
    pub fn get_current() -> Result<Option<String>> {
        let current_version_dir = crate::fs::directories::get_current_version_dir()
            .context("Failed to get current version directory.")?;

        if !current_version_dir.exists() {
            return Ok(None);
        }

        let current_version = match fs::canonicalize(&current_version_dir) {
            Ok(path) => path,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(None);
                } else {
                    println!("Failed to get current version: {}", e);
                    return Err(e.into());
                }
            }
        };

        let current_version = current_version
            .file_name()
            .ok_or_else(|| anyhow!("Failed to get current version."))?
            .to_str()
            .ok_or_else(|| anyhow!("Failed to get current version."))?
            .to_owned();

        Ok(Some(current_version))
    }

    /// Sets the given version of Dalamud as the currently active version.
    pub fn set_current(branch: &str) -> Result<()> {
        let versions_dir = get_versions_dir().context("Failed to get versions directory.")?;

        let branch_dir = versions_dir.join(branch);
        if !branch_dir.exists() {
            return Err(anyhow!("The branch {} is not installed.", branch));
        }

        // Remove the current version
        let current_version_dir = crate::fs::directories::get_current_version_dir()
            .context("Failed to get current version directory.")?;
        let _ = fs::remove_dir_all(&current_version_dir);

        // Make a symlink to the new version
        symlink::symlink_dir(&branch_dir, &current_version_dir)?;

        Ok(())
    }

    /// Unsets the currently active version of Dalamud.
    pub fn unset_current() -> Result<()> {
        let current_version_dir = crate::fs::directories::get_current_version_dir()
            .context("Failed to get current version directory.")?;

        if !current_version_dir.exists() {
            return Ok(());
        }

        symlink::remove_symlink_dir(&current_version_dir)?;

        Ok(())
    }
}
