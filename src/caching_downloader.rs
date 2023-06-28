use std::{
    collections::HashMap,
    fs, mem,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context};
use downloader::{Download, DownloadSummary, Downloader, Error};
use rand::random;

pub struct CachingDownloader {
    inner: Downloader,
    cache: PathBuf,
    base: PathBuf,
}

impl CachingDownloader {
    ///
    /// This downloader will put all downloaded files into a cache directory, before moving them to
    /// their actual target location. If a download fails, any corresponding file will not be
    /// touched.
    ///
    /// The `inner` is the downloader used to perform the actual download. The `cache` directory is
    /// the location to put the downloaded files in. Individual downloads will create subdirectories
    /// in it, to avoid conflicts. The `base` is the path relative file names are relative to.
    pub fn new(inner: Downloader, cache: &Path, base: &Path) -> anyhow::Result<Self> {
        let cache_ready = cache.is_dir();
        ensure!(
            cache_ready || !cache.exists(),
            "cache directory is neither directory nor nonexistant ({})",
            cache.display()
        );

        if !cache_ready {
            fs::create_dir_all(cache).context(format!(
                "failed to create cache directory ({})",
                cache.display()
            ))?;
        }

        ensure!(base.is_dir(), "base directory doesn't exist");

        Ok(Self {
            inner,
            cache: cache.to_path_buf(),
            base: base.to_path_buf(),
        })
    }

    pub fn download(
        &mut self,
        downloads: &mut [Download],
        partiton: Option<&str>,
    ) -> anyhow::Result<Vec<anyhow::Result<DownloadSummary>>> {
        let partition = partiton
            .map(ToString::to_string)
            .unwrap_or_else(|| format!("{:0>16x}", random::<u64>()));
        let cache = self.cache.join(partition);

        ensure!(!cache.exists(), "cache partition exists");
        fs::create_dir(&cache).context("failed to create cache partition")?;

        let mut mapping: HashMap<_, _> = downloads
            .iter_mut()
            .enumerate()
            .map(|(counter, down)| (cache.join(format!("{counter:0>16x}")), down))
            .map(|(cache, down)| (cache.clone(), mem::replace(&mut down.file_name, cache)))
            .collect();

        let mut results: Vec<_> = self
            .inner
            .download(downloads)
            .context("all downloads failed")?
            .into_iter()
            .map(|r| handle_file(&mut mapping, r, &self.base))
            .collect();

        for (leftover, _) in mapping {
            if let Err(err) =
                fs::remove_file(leftover).context("failed to delete leftover cache file")
            {
                results.push(Err(err));
            }
        }

        if let Err(err) = fs::remove_dir(cache).context("failed to delete cache partition") {
            results.push(Err(err));
        }

        Ok(results)
    }
}

fn handle_file(
    mapping: &mut HashMap<PathBuf, PathBuf>,
    summary: Result<DownloadSummary, Error>,
    base: &Path,
) -> anyhow::Result<DownloadSummary> {
    let mut summary = summary.context("download failed")?;

    let cache = mapping
        .remove(&summary.file_name)
        .context("unknown target location")?;
    let mut cache = base.join(cache);

    mem::swap(&mut summary.file_name, &mut cache);

    fs::hard_link(&cache, &summary.file_name)
        .or_else(|_| fs::copy(&cache, &summary.file_name).map(|_| ()))
        .context("failed to copy downloaded file to target location")?;

    fs::remove_file(cache).context("failed to remove cached file")?;

    Ok(summary)
}
