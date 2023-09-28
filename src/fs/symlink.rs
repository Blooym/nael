use anyhow::Result;
use std::path::Path;
use symlink::{remove_symlink_dir as remove_symlink_impl, symlink_dir as symlink_dir_impl};

/// Creates a simlink from the source to the destination.
///
/// This method is platform agnostic and will handle implementation detail differences between platforms.
pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    symlink_dir_impl(src, dst).map_err(|e| e.into())
}

/// Removes a symlink directory.
///
/// This method is platform agnostic and will handle implementation detail differences between platforms.
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    remove_symlink_impl(path).map_err(|e| e.into())
}
