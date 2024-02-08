use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fmt::Debug, fs::File, path::Path, time::Duration};
use zip::ZipArchive;

/// Extract an archive to a destination path, overwriting existing files.
///
/// # Errors
/// This function will return an error in the following situations, but is not limited to just these cases:
/// * When unable to open the archive at the archive path for reading.
/// * When unable to extract the archive to the destination path.
pub fn extract_with_progress_bar<P: AsRef<Path> + Debug, D: AsRef<Path> + Debug>(
    archive_path: &P,
    destination_path: &D,
) -> Result<()> {
    let extract_progress_bar = ProgressBar::new_spinner();
    extract_progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("[{elapsed_precise}] {spinner:.green} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    extract_progress_bar.set_message("Extracting archive...");
    extract_progress_bar.enable_steady_tick(Duration::from_millis(100));

    let mut archive = ZipArchive::new(
        File::open(archive_path)
            .with_context(|| format!("failed to open file {archive_path:?}"))?,
    )?;
    archive.extract(destination_path)?;

    extract_progress_bar.finish_and_clear();

    Ok(())
}
