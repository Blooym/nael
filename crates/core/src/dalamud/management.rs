use super::{sources::ReleaseSource, version_info::DalamudVersionInfo};
use crate::fs::{archive, storage::AppStorage};
use anyhow::{anyhow, Context, Result};
use serde::de::IgnoredAny;
use std::{
    fmt::Debug,
    fs::{self},
    path::PathBuf,
    sync::Arc,
};
use tempfile::tempdir;

/// Information about a Dalamud branch install & utilities for managing all installations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DalamudInstallation<S: AppStorage> {
    /// The name of the branch this installation represents.
    pub branch_name: String,

    /// The storage implementation to use for all operations with this installation.
    storage: Arc<S>,
}

impl<S: AppStorage> DalamudInstallation<S> {
    /// Downloads & extracts the remote branch & version information to [`AppStorage::get_branch_directory`].
    ///
    /// # Notes
    /// This function will attempt to prevent cases where a local installation is deleted when a remote version download fails.
    /// If there is already an installation of the given branch locally it will be overwritten only when the archive is being extracted.
    async fn download_branch_impl<RS: ReleaseSource>(
        branch_name: &str,
        storage: &Arc<S>,
        release_source: &RS,
    ) -> Result<DalamudInstallation<S>> {
        let branch_directory = storage.get_branch_directory(branch_name)?;
        let version_info_path = storage.get_branch_version_info_path(branch_name)?;

        // Download release archive.
        let work_dir = tempdir().context("creation temporary working directory failed")?;
        let download_path = work_dir.path().join("dalamud.zip");
        release_source
            .get_release_archive_file(branch_name)
            .download_with_progress_bar(&download_path)
            .await
            .context("release archive download failure")?;

        // Extract release archive - delete existing install if found.
        if branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            fs::remove_dir_all(&branch_directory).with_context(|| {
                format!("failed to delete existing branch directory {branch_directory:?}",)
            })?;
        }
        archive::extract_with_progress_bar(&download_path, &branch_directory)
            .context("failed to extract release archive to disk")?;
        drop(work_dir); // Deletes the temporary directory.

        // Download version information.
        if let Err(err) = release_source
            .get_version_info_file(branch_name)
            .download_with_progress_bar(&version_info_path)
            .await
        {
            eprintln!("Warning: Unable to obtain version information: {err:?}");
            eprintln!(
                "This branch will not be able to compare its version against the release source later."
            );
        };

        Ok(DalamudInstallation {
            storage: Arc::clone(storage),
            branch_name: branch_name.to_owned(),
        })
    }

    /// Create a new branch installation with the given storage.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When the branch already exists locally.
    /// * When unable to check if the given branch directory exists.
    /// * When any part of the installation process fails.
    pub async fn create<RS: ReleaseSource>(
        branch_name: &str,
        storage: &Arc<S>,
        release_source: &RS,
    ) -> Result<DalamudInstallation<S>> {
        let branch_directory = storage.get_branch_directory(branch_name)?;
        if branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Err(anyhow!(
                "branch {} already exists in storage, obtain an instance and call the update method instead",
                branch_name,
            ));
        }
        Self::download_branch_impl(branch_name, storage, release_source).await
    }

    /// Remove the branch installation from storage.
    ///
    /// If the branch installation is set as 'active' it will automatically be unset via [`DalamudInstallation::unset_active`].
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to check if the given branch directory exists.
    /// * When the branch directory cannot be found.
    pub fn remove(self) -> Result<()> {
        let branch_directory = self.storage.get_branch_directory(&self.branch_name)?;
        if !branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Err(anyhow!(
                "unable to find branch {} in versions directory",
                &self.branch_name
            ));
        }

        if self
            .is_active()
            .context("failed to check if branch to remove is active")?
        {
            Self::unset_active(&self.storage)
                .context("failed unsetting removed branch as active")?;
        }

        fs::remove_dir_all(&branch_directory)
            .with_context(|| format!("failed to remove branch directory {branch_directory:?}"))?;

        Ok(())
    }

    /// Update the branch installation to the latest remote version, consuming the instance and returning a new one when [`Ok`].
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When any filesystem operation fails.
    /// * When any part of the installation process fails.
    ///
    /// # Notes
    /// This function consumes the calling instance for safety.
    /// If this operation returns an [`Err`] then it is possible that the original installation files may no longer be present on disk.
    /// This would cause undesired behaviour with other operations.
    ///
    /// In cases where it is desired to reobtain the installation call [`DalamudInstallation::get`]; This will validate the install
    /// is still valid before passing back an instance that can be used again.
    ///
    /// # Recommendations
    /// * Compare [`DalamudInstallation::get_version_info`] with [`DalamudInstallation::get_remote_version_info`] when available
    /// to check if the installation actually needs to be updated before trying to update.
    pub async fn update<RS: ReleaseSource>(self, release_source: &RS) -> Result<Self> {
        Self::download_branch_impl(&self.branch_name, &self.storage, release_source).await
    }

    /// Check a branch installation exists in the given storage.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to check if the given branch directory exists.
    pub fn exists(branch_name: &str, storage: &Arc<S>) -> Result<bool> {
        let branch_directory = storage.get_branch_directory(branch_name)?;
        if !branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// Get a branch installation from the given storage.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to check if the given branch directory exists.
    pub fn get(branch_name: &str, storage: &Arc<S>) -> Result<Option<DalamudInstallation<S>>> {
        let branch_directory = storage.get_branch_directory(branch_name)?;
        if !branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Ok(None);
        }

        Ok(Some(DalamudInstallation {
            storage: Arc::clone(storage),
            branch_name: branch_name.to_owned(),
        }))
    }

    /// Get all installations in the given storage.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to check if the versions directory exists.
    /// * When unable to read relevant directories.
    pub fn get_all(storage: &Arc<S>) -> Result<Option<Vec<DalamudInstallation<S>>>> {
        let branch_directory = storage.get_branches_directory()?;
        if !branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Ok(None);
        }

        let versions = fs::read_dir(&branch_directory)
            .with_context(|| format!("failed to read {branch_directory:?}"))?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let file_name = path.file_name()?.to_str()?.to_owned();

                Some(DalamudInstallation {
                    storage: Arc::clone(storage),
                    branch_name: file_name,
                })
            })
            .collect::<Vec<_>>();

        if versions.is_empty() {
            return Ok(None);
        }

        Ok(Some(versions))
    }

    /// Get the active branch by resolving the relevant storage symlink.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to read the active version symlink for any reason other than [`std::io::ErrorKind::NotFound`].
    /// * When canonicalizing the symlink fails for any reason other than [`std::io::ErrorKind::NotFound`].
    /// * When converting the directory name from [`std::ffi::OsStr`] to [`String`].
    pub fn get_active(storage: &Arc<S>) -> Result<Option<DalamudInstallation<S>>> {
        let active_branch_symlink = storage.get_active_branch_symlink()?;

        if let Err(err) = fs::read_link(&active_branch_symlink) {
            return match err.kind() {
                std::io::ErrorKind::NotFound => Ok(None),
                _ => Err(err.into()),
            };
        };

        let active_branch = match fs::canonicalize(&active_branch_symlink) {
            Ok(path) => path,
            Err(err) => {
                return match err.kind() {
                    std::io::ErrorKind::NotFound => Ok(None),
                    _ => Err(err.into()),
                };
            }
        };

        let branch_name = active_branch
            .file_name()
            .context("branch directory name was None after confirming existence")?
            .to_str()
            .ok_or_else(|| anyhow!("result of conversion from OsStr to &str was None"))?
            .to_owned();

        Ok(Some(DalamudInstallation {
            storage: Arc::clone(storage),
            branch_name,
        }))
    }

    /// Set the branch installation as active by setting up the relevant storage symlinks.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to check if the branch directory exists.
    /// * When the branch directory does not exist.
    /// * When removing the existing active version symlink fails for any reason other than [`std::io::ErrorKind::NotFound`].
    pub fn set_active(&self) -> Result<()> {
        let branch_directory = self.storage.get_branch_directory(&self.branch_name)?;
        if !branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Err(anyhow!(
                "unable to find branch {} in versions directory",
                self.branch_name
            ));
        }

        // Remove the active version and make a new symlink.
        let active_version_location = self.storage.get_active_branch_symlink()?;
        if let Err(err) = symlink::remove_symlink_dir(&active_version_location) {
            match err.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => return Err(err.into()),
            }
        };
        symlink::symlink_dir(&branch_directory, &active_version_location).with_context(|| {
            if cfg!(windows) {
                // Output a windows-specific error message prompting about developer mode as symlinks are considered a developer/priviledged
                // action. https://security.stackexchange.com/questions/10194/why-do-you-have-to-be-an-admin-to-create-a-symlink-in-windows
                format!(
                "failed to create symlink from {branch_directory:?} to {active_version_location:?} (have you enabled Windows Developer Mode or run as an administrator?)"
            )
            } else {
                format!(
                "failed to create symlink from {branch_directory:?} to {active_version_location:?}"
            )
            }
        })?;

        Ok(())
    }

    /// Unsets the active branch installation if one is set. When one is not set, the function will return [`Ok`] anyway.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to read the active version symlink for any reason other than [`std::io::ErrorKind::NotFound`].
    /// * When removing the active version symlink fails for any reason other than [`std::io::ErrorKind::NotFound`].
    pub fn unset_active(storage: &S) -> Result<()> {
        let active_branch_symlink = storage.get_active_branch_symlink()?;

        if let Err(err) = fs::read_link(&active_branch_symlink) {
            return match err.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(err.into()),
            };
        };

        if let Err(err) = symlink::remove_symlink_dir(&active_branch_symlink) {
            return match err.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(err.into()),
            };
        };

        Ok(())
    }

    /// Check if the branch installation is set as active.
    ///
    /// This is currently a convinence function for checking 'self == [`DalamudInstallation::get_active`]'.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When the call to [`DalamudInstallation::get_active`] fails.
    pub fn is_active(&self) -> Result<bool> {
        let Some(active) = Self::get_active(&self.storage)
            .context("failed to check for the currently active branch")?
        else {
            return Ok(false);
        };
        Ok(self.branch_name == active.branch_name)
    }

    /// Get the directory for the branch installation. Will return [`None`] if the directory does not exist.
    ///
    /// This is a safety wrapper around [`AppStorage::get_branch_directory`].
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When unable to check if the branch directory exists.
    pub fn get_location(&self) -> Result<Option<PathBuf>> {
        let branch_directory = self.storage.get_branch_directory(&self.branch_name)?;

        if !branch_directory
            .try_exists()
            .with_context(|| format!("unable to check existence of {branch_directory:?}"))?
        {
            return Ok(None);
        }

        Ok(Some(branch_directory))
    }

    /// Get the version information for the branch installation by checking the storage to find release version info
    /// file.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When the returned version info is not valid JSON.
    pub fn get_version_info(&self) -> Result<Option<DalamudVersionInfo>> {
        Ok(Some(DalamudVersionInfo::from_path_ref(
            &self
                .storage
                .get_branch_version_info_path(&self.branch_name)?,
        )?))
    }

    /// Get the remote version information for the branch installation using the given release source.
    ///
    /// # Errors
    /// * When a network failure occurs fetching the remote version information.
    /// * When the returned version info is not valid JSON.
    // FIXME: Remove the requirement on &self reference and AppStorage bound so this can be called without a local instance.
    pub async fn get_remote_version_info<RS: ReleaseSource>(
        &self,
        release_source: &RS,
    ) -> Result<Option<DalamudVersionInfo>> {
        Ok(Some(
            release_source
                .get_version_info_file(&self.branch_name)
                .read_to_string()
                .await?
                .parse::<DalamudVersionInfo>()?,
        ))
    }

    /// Get the version information for the branch installation by checking the storage to find release version info
    /// and returning it without serialization into [`DalamudVersionInfo`]
    ///
    /// # Notes
    /// The [`String`] inside of [`Some`] is guaranteed to always be valid JSON as it is validated before being returned.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any [`AppStorage`] operation fails.
    /// * When the returned version info string is not valid JSON.
    pub fn get_version_info_json(&self) -> Result<Option<String>> {
        let version_info_path = self
            .storage
            .get_branch_version_info_path(&self.branch_name)?;

        if !version_info_path
            .try_exists()
            .with_context(|| format!("unable to check existence of {version_info_path:?}"))?
        {
            return Ok(None);
        }

        let content = fs::read_to_string(&version_info_path)
            .with_context(|| format!("could not read file at {version_info_path:?}"))?
            .trim()
            .to_owned();

        if content.is_empty() {
            return Ok(None);
        }

        // Validate that the file is actually valid JSON.
        let _: IgnoredAny = serde_json::from_str(&content)?;

        Ok(Some(content))
    }

    /// Get the remote version information for the branch installation by using the given release source.
    /// and returning it without serialization into [`DalamudVersionInfo`]
    ///
    /// # Notes
    /// The [`String`] inside of [`Some`] is guaranteed to always be valid JSON as it is validated before being returned.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When a network failure occurs fetching the remote version information.
    /// * When the returned version info is not valid JSON.
    // FIXME: Remove the requirement on &self reference and AppStorage bound so this can be called without a local instance.
    pub async fn get_remote_version_info_json<RS: ReleaseSource>(
        &self,
        release_source: &RS,
    ) -> Result<Option<String>> {
        let version_info_raw: String = release_source
            .get_version_info_file(&self.branch_name)
            .read_to_string()
            .await?;

        // Validate that the returned response is actually valid JSON.
        let _: IgnoredAny = serde_json::from_str(&version_info_raw)?;

        Ok(Some(version_info_raw))
    }
}
