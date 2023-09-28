use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, path, time::Duration};
use zip::ZipArchive;

/// The template for the progress bar used when extracting the Dalamud archive.
const EXTRACT_PROGRESS_BAR_TEMPLATE: &str = "[{elapsed_precise}] {spinner:.green} {msg}";

/// Extracts the given file to the given directory.
pub fn extract_archive_with_progress(
    path: &path::PathBuf,
    extract_dir: &path::PathBuf,
) -> Result<()> {
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
