use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{anyhow, Context};
use downloader::Downloader;
use serde::{Deserialize, Serialize};

use crate::{PROJ_DIRS, USER_DIRS};

/// Global configuration values.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    /// The base directory used for resolving relative paths. Defaults to the home directory.
    pub base_directory: PathBuf,
    /// The number of parallel downloads. Defaults to 32.
    pub parallel_downloads: u16,
    /// The number of retries. Defaults to 3.
    pub retries: u16,
    /// The timeout for establishing a connection in seconds. Defaults to 15.
    pub timeout_connection: u64,
    /// The timeout for downloading a file in seconds. Defaults to 30.
    pub timeout_download: u64,
    /// The user agent to use for HTTP communication. Default is not stabilized.
    pub user_agent: String,
}

impl Config {
    /// Read the configuration from the default file, or revert to default values if unavailable.
    pub fn read_from_default_file() -> anyhow::Result<Self> {
        let dirs = PROJ_DIRS.get().expect("directories not initialized");

        let mut path = dirs.config_dir().to_path_buf();
        path.push("config.toml");

        if path.is_file() {
            Self::read_from_file(&path)
        } else if !path.exists() {
            Ok(Self::default())
        } else {
            Err(anyhow!(
                "default configuration file is neither file nor nonexistent"
            ))
        }
    }

    /// Read the configuration from the specified file.
    pub fn read_from_file(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path).context("failed to open config file")?;
        let mut buf = BufReader::new(file);

        let mut data = String::new();
        buf.read_to_string(&mut data)
            .context("failed to read config data")?;

        toml::from_str(&data).context("failed to parse config file")
    }

    /// Create a [`Downloader`] with the configuration applied.
    pub fn downloader(&self) -> anyhow::Result<Downloader> {
        Downloader::builder()
            .user_agent(&self.user_agent)
            .connect_timeout(Duration::from_secs(self.timeout_connection))
            .timeout(Duration::from_secs(self.timeout_download))
            .parallel_requests(self.parallel_downloads)
            .retries(self.retries)
            .download_folder(&self.base_directory)
            .build()
            .context("failed to build downloader")
    }
}

impl Default for Config {
    fn default() -> Self {
        let dirs = USER_DIRS.get().expect("directories not initialized");

        Self {
            base_directory: dirs.home_dir().to_path_buf(),
            parallel_downloads: 32,
            retries: 3,
            timeout_connection: 15,
            timeout_download: 30,
            user_agent: format!(
                "{} {}.{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR")
            ),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "base directory:     {}", self.base_directory.display())?;
        writeln!(f, "parallel downloads: {}", self.parallel_downloads)?;
        writeln!(f, "retries:            {}", self.retries)?;
        writeln!(f, "timeout connection: {}s", self.timeout_connection)?;
        writeln!(f, "timeout download:   {}s", self.timeout_download)?;
        write!(f, "user agent:         {}", self.user_agent)
    }
}
