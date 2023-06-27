use std::{
    fmt::Display,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use anyhow::{bail, ensure, Context};
use indoc::writedoc;
use serde::{Deserialize, Serialize};

use crate::PROJ_DIRS;

const PERSISTENT_FILE: &str = "persistent_state.json";

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct PersistentState {
    list: Option<String>,
    target: Option<usize>,
}

impl PersistentState {
    pub fn list(&self) -> Option<&str> {
        self.list.as_deref()
    }

    pub fn set_list(&mut self, list: &str) {
        self.list = Some(list.to_string());
        self.target = None;
    }

    pub fn clear_list(&mut self) {
        self.list = None;
        self.target = None;
    }

    pub fn target(&self) -> Option<usize> {
        self.target
    }

    pub fn set_target(&mut self, index: usize) {
        if self.list.is_some() {
            self.target = Some(index);
        }
    }
}

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
            .map(BufReader::new)
            .and_then(Self::read_from)
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
            .map(BufWriter::new)
            .and_then(|w| self.save_to(w))
    }

    pub fn save_to(&self, writer: impl Write) -> anyhow::Result<()> {
        serde_json::to_writer(writer, &self).context("failed to write persistent state file")
    }
}

impl Display for PersistentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            "
            current list:   {} 
            current target: {}",
            self.list.as_ref().map(AsRef::as_ref).unwrap_or("none"),
            self.target
                .map(|t| t.to_string())
                .unwrap_or("none".to_string())
        )
    }
}
