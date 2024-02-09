use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs::File, io::Write, path::Path};

/// Represents a remote resource with convinence methods attached.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RemoteResource {
    pub url: String,
}

impl RemoteResource {
    /// Create a new resource from the given URL.
    pub fn from_url(url: String) -> Self {
        Self { url }
    }

    /// Read the entire contents of the resource to a [`String`].
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When a network request fails.
    pub async fn read_into_string(&self) -> Result<String> {
        let download = reqwest::get(&self.url)
            .await
            .with_context(|| format!("failed to read remote file at {}", self.url))?;
        Ok(download.text().await?)
    }

    /// Download the resource at the underlying url and write it to the disk at given path.
    ///
    /// This method will output a progress bar to stderr.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When a network request fails.
    /// * When any filesystem operation fails.
    pub async fn download_with_progress_bar<P: AsRef<Path> + Debug>(
        &self,
        path: P,
    ) -> Result<File> {
        let mut download = reqwest::get(&self.url).await?;

        let download_progress_bar = ProgressBar::new(download.content_length().unwrap_or(0));
        download_progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        download_progress_bar.set_message(self.url.clone());

        if !download.status().is_success() {
            download_progress_bar.finish_and_clear();
            return Err(anyhow!(
                "Network request to {} failed: {}",
                self.url,
                download.status()
            ));
        }

        let mut file =
            File::create(&path).with_context(|| format!("failed creating file {path:?}",))?;

        while let Some(chunk) = download.chunk().await? {
            file.write_all(&chunk)
                .context("writing chunk to disk failed")?;
            download_progress_bar.inc(chunk.len() as u64);
        }
        download_progress_bar.finish_and_clear();

        Ok(file)
    }
}
