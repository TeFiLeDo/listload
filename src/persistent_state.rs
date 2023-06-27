use std::{
    fmt::Display,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use anyhow::{bail, ensure, Context};
use serde::{Deserialize, Serialize};

use crate::PROJ_DIRS;

const PERSISTENT_FILE: &str = "persistent_state.json";

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct PersistentState {}

impl PersistentState {
    pub fn read_from_default_file() -> anyhow::Result<Self> {
        let dirs = PROJ_DIRS.get().expect("directories not initialized");

        let mut path = dirs.preference_dir().to_path_buf();
        path.push(PERSISTENT_FILE);

        if path.is_file() {
            Self::read_from_file(&path)
        } else if !path.exists() {
            Ok(Self::default())
        } else {
            bail!("persistent state file is neither file nor nonexistant")
        }
    }

    pub fn read_from_file(path: &Path) -> anyhow::Result<Self> {
        File::open(path)
            .context("failed to open persistent state file")
            .map(|r| BufReader::new(r))
            .and_then(|r| Self::read_from(r))
    }

    pub fn read_from(reader: impl Read) -> anyhow::Result<Self> {
        serde_json::from_reader(reader).context("failed to parse persistent state file")
    }

    pub fn save_to_default_file(&self) -> anyhow::Result<()> {
        let dirs = PROJ_DIRS.get().expect("directories not initialized");

        let mut path = dirs.preference_dir().to_path_buf();
        ensure!(
            path.is_dir() || !path.exists(),
            "preference directory is neither directory nor nonexistent"
        );

        if !path.exists() {
            create_dir_all(&path).context("failed to create program preference directory")?;
        }

        path.push(PERSISTENT_FILE);
        ensure!(
            path.is_file() || !path.exists(),
            "persistent state file is neither file nor nonexistant"
        );

        self.save_to_file(&path)
    }

    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        File::create(path)
            .context("failed to create persistent state file")
            .map(|w| BufWriter::new(w))
            .and_then(|w| self.save_to(w))
    }

    pub fn save_to(&self, writer: impl Write) -> anyhow::Result<()> {
        serde_json::to_writer(writer, &self).context("failed to write persistent state file")
    }
}

impl Display for PersistentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no persistent state yet")
    }
}
