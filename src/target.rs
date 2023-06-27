use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::ensure;
use downloader::Download;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    urls: Vec<Url>,
    file: PathBuf,
    comment: Option<String>,
}

impl Target {
    pub fn new(urls: Vec<Url>, file: &Path, comment: Option<&str>) -> anyhow::Result<Self> {
        ensure!(!urls.is_empty(), "at least one url is required");
        ensure!(
            file.is_file() || !file.exists(),
            "file must be file or nonexistent"
        );

        for url in &urls {
            let scheme = url.scheme();
            ensure!(scheme == "http" || scheme == "https", "url is not http(s)");
        }

        Ok(Self {
            urls,
            file: file.to_path_buf(),
            comment: comment.map(ToString::to_string),
        })
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.comment
                .clone()
                .map(|c| format!("{c}: "))
                .unwrap_or_default(),
            self.file.display()
        )
    }
}

impl From<&Target> for Download {
    fn from(value: &Target) -> Self {
        match value.urls.len() {
            0 => panic!("target without url"),
            1 => Download::new(value.urls[0].as_str()),
            _ => Download::new_mirrored(
                value
                    .urls
                    .iter()
                    .map(|u| u.as_str())
                    .collect::<Vec<_>>()
                    .as_ref(),
            ),
        }
        .file_name(&value.file)
    }
}
