use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

/// A storage implemenation that nael uses to manage its state and data.
pub trait AppStorage: Clone {
    /// Get the raw local version info filename.
    ///
    /// If you are using this for manually joining with the branch path you should use [`AppStorage::get_branch_version_info_path()`] instead.
    fn get_version_info_filename(&self) -> &str;

    /// Get a [`PathBuf`] to the symlink that links to the active branch of Dalamud.
    ///
    /// This will automatically create all leading directories apart from the symlink.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When there is no valid home directory found.
    /// * When creating any leading directory fails.
    fn get_active_branch_symlink(&self) -> Result<PathBuf>;

    /// Get a [`PathBuf`] of the directory that contains installed branches of Dalamud.
    ///
    /// This will automatically create all missing directories.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When there is no valid home directory found.
    /// * When creating any leading directory fails.
    fn get_branches_directory(&self) -> Result<PathBuf>;

    /// Get a [`PathBuf`] to where a specific branch *should* be stored.
    ///
    /// This will automatically create all missing directories apart from the branch directory itself.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When there is no valid home directory found.
    /// * When creating any leading directory fails.
    fn get_branch_directory(&self, branch_name: &str) -> Result<PathBuf>;

    /// Get a [`PathBuf`] to where the version info file is stored for the given branch.
    ///
    /// This will automatically create all missing directories apart from the branch directory.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When there is no valid home directory found.
    /// * When creating any leading directory fails.
    fn get_branch_version_info_path(&self, branch_name: &str) -> Result<PathBuf>;
}

/// The name of the sub-directory that contains installed branches of Dalamud.
//
//  Warning:
//  Any changes to this will break compatibility with existing installations.
const DALAMUD_BRANCHES_DIRNAME: &str = "dalamud-branches";
const DALAMUD_BRANCHES_DIRNAME_OLD: &str = "dalamud-versions"; // Temporary

/// The name of the symlink to the active branch of Dalamud
//  Warning:
//  Any changes to this will break compatibility with existing installations.
const ACTIVE_DALAMUD_VERSION_DIRNAME: &str = "active";

/// The name of the version info file contained inside of every release locally that has one available.
//  Warning:
//  Any changes to this will break compatibility with existing installations.
const VERSIONINFO_FILENAME: &str = "release.versiondata";

/// An operating system standards compliant disk-storage based implementation of [`AppStorage`].
///
/// Follows these standards for each operating system:
/// * `Windows` [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx)
/// * `MacOS`: [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
/// * `Linux`: [XDG-Base](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and [XDG-User](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)
#[derive(Debug, Clone, Copy)]
pub struct CompliantDiskStorage {
    qualifier: &'static str,
    organization: &'static str,
    name: &'static str,
}

impl CompliantDiskStorage {
    pub fn new(qualifier: &'static str, organization: &'static str, name: &'static str) -> Self {
        Self {
            qualifier,
            organization,
            name,
        }
    }

    /// Convinence function for [`directories::ProjectDirs::from`].
    fn get_project_dir(&self) -> Result<ProjectDirs> {
        directories::ProjectDirs::from(self.qualifier, self.organization, self.name)
            .context("No valid home directory path could be retrieved from the operating system")
    }

    /// Get a [`PathBuf`] of the app's base data directory.
    ///
    /// Automatically creates all missing directories.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When there is no valid home directory found.
    /// * When creating any leading directory fails.
    fn get_app_data_dir(&self) -> Result<PathBuf> {
        let project_dir = self.get_project_dir()?;
        create_dir_all(project_dir.data_dir()).context("data directory creation failed")?;
        Ok(project_dir.data_dir().to_path_buf())
    }

    /// Get a [`PathBuf`] of the app's base config directory.
    ///
    /// Automatically creates all missing directories.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When there is no valid home directory found.
    /// * When creating any leading directory fails.
    fn get_app_config_dir(&self) -> Result<PathBuf> {
        let project_dir = self.get_project_dir()?;
        create_dir_all(project_dir.config_dir()).context("config directory creation failed")?;
        Ok(project_dir.config_dir().to_path_buf())
    }
}

impl AppStorage for CompliantDiskStorage {
    fn get_version_info_filename(&self) -> &str {
        VERSIONINFO_FILENAME
    }

    fn get_active_branch_symlink(&self) -> Result<PathBuf> {
        let config_dir: PathBuf = self.get_app_config_dir()?;
        Ok(config_dir.join(ACTIVE_DALAMUD_VERSION_DIRNAME))
    }

    fn get_branches_directory(&self) -> Result<PathBuf> {
        let branches_dir: PathBuf = self.get_app_data_dir()?.join(DALAMUD_BRANCHES_DIRNAME);
        create_dir_all(&branches_dir).context("branches directory creation failed")?;

        // Temporary cleanup of old versions due to a rename.
        // TODO: Remove this in a few releases time.
        let _ = fs::remove_dir_all(self.get_app_data_dir()?.join(DALAMUD_BRANCHES_DIRNAME_OLD));

        Ok(branches_dir)
    }

    fn get_branch_directory(&self, branch_name: &str) -> Result<PathBuf> {
        Ok(self.get_branches_directory()?.join(branch_name))
    }

    fn get_branch_version_info_path(&self, branch_name: &str) -> Result<PathBuf> {
        Ok(self
            .get_branch_directory(branch_name)?
            .join(self.get_version_info_filename()))
    }
}
