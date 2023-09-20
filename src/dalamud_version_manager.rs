use crate::{
    directories::get_versions_dir,
    repository::{Repository, DEFAULT_DISTRIB_REPO_FILE},
};
use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{self, File},
    io::Write,
    path,
    time::Duration,
};
use tempfile::tempdir;
use zip::ZipArchive;

const DOWNLOAD_PROGRESS_BAR_TEMPLATE: &str =
    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}";
const EXTRACT_PROGRESS_BAR_TEMPLATE: &str = "[{elapsed_precise}] {spinner:.green} {msg}";

/// A struct that contains logic for interacting with Dalamud versions.
#[derive(Debug)]
pub struct DalamudVersionManager {}

impl DalamudVersionManager {
    /// Downloads the file at the given URL and writes it to the given path.
    async fn download_file_with_progress(url: String, path: path::PathBuf) -> Result<()> {
        // Create the download request and setup a progress bar.
        let mut download = reqwest::get(&url).await?;
        let download_progress_bar = ProgressBar::new(download.content_length().unwrap_or(0));
        download_progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(DOWNLOAD_PROGRESS_BAR_TEMPLATE)
                .unwrap()
                .progress_chars("#>-"),
        );
        download_progress_bar.set_message(url.clone());

        // Check if the request was successful.
        if !download.status().is_success() {
            download_progress_bar.finish_and_clear();
            return Err(anyhow!(
                "An error occured whilst trying to download from {}: {}",
                url,
                download.status()
            ));
        }

        // Write the downloaded data to the file and update the progress bar.
        let mut file = File::create(&path)?;
        while let Some(chunk) = download.chunk().await? {
            file.write_all(&chunk)?;
            download_progress_bar.inc(chunk.len() as u64);
        }
        download_progress_bar.finish_and_clear();

        Ok(())
    }

    /// Extracts the given file to the given directory.
    fn extract_file_with_progress(path: path::PathBuf, extract_dir: path::PathBuf) -> Result<()> {
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
        archive.extract(&extract_dir)?;
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
        let url = Repository::default().get_download_url(branch);
        let dir = tempdir()?;
        let file_path = dir.path().join(DEFAULT_DISTRIB_REPO_FILE);

        // Download & extract the archive.
        Self::download_file_with_progress(url, file_path.clone()).await?;
        Self::extract_file_with_progress(file_path, extract_dir)?;

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
        let current_version_dir = crate::directories::get_current_version_dir()
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
        let current_version_dir = crate::directories::get_current_version_dir()
            .context("Failed to get current version directory.")?;
        let _ = fs::remove_dir_all(&current_version_dir);

        // Make a symlink to the new version
        symlink::symlink_dir(&branch_dir, &current_version_dir)?;

        Ok(())
    }

    /// Unsets the currently active version of Dalamud.
    pub fn unset_current() -> Result<()> {
        let current_version_dir = crate::directories::get_current_version_dir()
            .context("Failed to get current version directory.")?;

        if !current_version_dir.exists() {
            return Ok(());
        }

        symlink::remove_symlink_dir(&current_version_dir)?;

        Ok(())
    }
}
