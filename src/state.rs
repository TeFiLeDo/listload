use std::{fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    active_list: String,
    #[serde(skip, default)]
    changed: bool,
}

impl State {
    pub fn active_list(&self) -> &str {
        &self.active_list
    }

    pub fn set_active_list(&mut self, active_list: String) {
        self.active_list = active_list;
        self.changed = true;
    }

    pub fn changed(&self) -> bool {
        self.changed
    }
}

impl State {
    pub fn load(directory: &Path) -> anyhow::Result<Self> {
        Self::load_file(&directory.join("state.json"))
    }

    pub fn load_file(file: &Path) -> anyhow::Result<Self> {
        if !file.is_file() {
            return Ok(Self::default());
        }

        serde_json::from_slice(&fs::read(file).context("failed to read state file")?)
            .context("failed to deserialize state")
    }

    pub fn save(&mut self, directory: &Path) -> anyhow::Result<()> {
        self.save_file(&directory.join("state.json"))
    }

    pub fn save_file(&mut self, file: &Path) -> anyhow::Result<()> {
        self.changed = false;
        fs::write(
            file,
            serde_json::to_vec(&self).context("failed to serialize state")?,
        )
        .context("failed to write state file")
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_list: String::from("default"),
            changed: false,
        }
    }
}
