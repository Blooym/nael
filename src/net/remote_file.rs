use anyhow::{anyhow, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, io::Write, path};

/// The template that will be used by indicatif's [`ProgressStyle`] when displaying the progress bar.
const DOWNLOAD_PROGRESS_BAR_TEMPLATE: &str =
    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}";

/// A struct that represents
pub struct RemoteFile {
    pub url: String,
}

impl RemoteFile {
    /// Creates a new RemoteFile from the given URL.
    pub fn from_url(url: String) -> Self {
        Self { url }
    }

    /// Downloads the file at the given URL and writes it to the given path.
    ///
    /// This will also output a progress bar to stderr to show the progress of the download.
    pub async fn download_with_progress(&self, path: path::PathBuf) -> Result<()> {
        // Create the download request and setup a progress bar.
        let mut download = reqwest::get(&self.url).await?;
        let download_progress_bar = ProgressBar::new(download.content_length().unwrap_or(0));
        download_progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(DOWNLOAD_PROGRESS_BAR_TEMPLATE)
                .unwrap()
                .progress_chars("#>-"),
        );
        download_progress_bar.set_message(self.url.clone());

        // Check if the request was successful.
        if !download.status().is_success() {
            download_progress_bar.finish_and_clear();
            return Err(anyhow!(
                "Network request to {} failed: {}",
                self.url,
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
}
