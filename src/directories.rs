use std::{fs, path::PathBuf};

/// The name of the program's subfolder in any directory returned by [`dirs`] (such as [`dirs::data_dir`] or [`dirs::cache_dir`]).
///
/// # Warning
/// Changing this will break compatibility with any existing installations as the program will not be able to find any
/// existing data created by previous versions.
const DIR_SUBFOLDER_NAME: &str = "nael";

/// Returns the path to the program's data directory.
pub fn get_data_dir() -> Option<PathBuf> {
    fs::create_dir_all(dirs::data_dir()?.join(DIR_SUBFOLDER_NAME)).ok()?;
    dirs::data_dir().map(|data_dir| data_dir.join(DIR_SUBFOLDER_NAME))
}

/// Returns the path to the program's config directory.
pub fn get_config_dir() -> Option<PathBuf> {
    fs::create_dir_all(dirs::config_dir()?.join(DIR_SUBFOLDER_NAME)).ok()?;
    dirs::config_dir().map(|config_dir| config_dir.join(DIR_SUBFOLDER_NAME))
}

/// The name of the subfolder that contains all installed versions of Dalamud.
const DATA_DIR_VERSIONS_FOLDER: &str = "dalamud-versions";

/// Returns the path to the program's Dalamud versions directory.
pub fn get_versions_dir() -> Option<PathBuf> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(data_dir.join(DATA_DIR_VERSIONS_FOLDER)).ok()?;
    Some(data_dir.join(DATA_DIR_VERSIONS_FOLDER))
}

/// The name of the subfolder that contains the currently active version of Dalamud.
const CURRENT_VERSION_SUBFOLDER: &str = "current";

/// Returns the path to the program's current version directory.
pub fn get_current_version_dir() -> Option<PathBuf> {
    let config_dir = get_config_dir()?;
    fs::create_dir_all(config_dir.join(CURRENT_VERSION_SUBFOLDER)).ok()?;
    Some(config_dir.join(CURRENT_VERSION_SUBFOLDER))
}
