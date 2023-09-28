use std::{fs, path::PathBuf};

/// The name of the program's sub-directory in data directories such as [`dirs::data_dir`] or [`dirs::cache_dir`].
///
/// # Warning
/// Changing this will break compatibility with any existing installations.
const APP_SUBDIR_NAME: &str = "nael";

/// The name of the sub-directory that contains all installed versions of Dalamud.
///
/// # Warning
/// Changing this will break compatibility with any existing installations.
const VERSIONS_DIR_NAME: &str = "dalamud-versions";

/// The name of the sub-directory that contains the currently active version of Dalamud.
///
/// # Warning
/// Changing this will break compatibility with any existing installations.
const CURRENT_VERSION_DIR_NAME: &str = "current";

/// Returns the path to the program's data directory, which is [`dirs::data_dir`] joined with [`APP_SUBDIR_NAME`].
pub fn get_app_data_dir() -> Option<PathBuf> {
    fs::create_dir_all(dirs::data_dir()?.join(APP_SUBDIR_NAME)).ok()?;
    dirs::data_dir().map(|data_dir| data_dir.join(APP_SUBDIR_NAME))
}

/// Returns the path to the program's config directory, which is [`dirs::config_dir`] joined with [`APP_SUBDIR_NAME`].
pub fn get_app_config_dir() -> Option<PathBuf> {
    fs::create_dir_all(dirs::config_dir()?.join(APP_SUBDIR_NAME)).ok()?;
    dirs::config_dir().map(|config_dir| config_dir.join(APP_SUBDIR_NAME))
}

/// Returns the path to the program's Dalamud versions directory, which is [`get_app_data_dir`] joined with [`VERSIONS_DIR_NAME`].
pub fn get_versions_dir() -> Option<PathBuf> {
    let data_dir = get_app_data_dir()?;
    fs::create_dir_all(data_dir.join(VERSIONS_DIR_NAME)).ok()?;
    Some(data_dir.join(VERSIONS_DIR_NAME))
}

/// Returns the path to the program's current version directory, which is [`get_app_config_dir`] joined with [`CURRENT_VERSION_DIR_NAME`].
pub fn get_current_version_dir() -> Option<PathBuf> {
    let config_dir = get_app_config_dir()?;
    fs::create_dir_all(config_dir.join(CURRENT_VERSION_DIR_NAME)).ok()?;
    Some(config_dir.join(CURRENT_VERSION_DIR_NAME))
}
