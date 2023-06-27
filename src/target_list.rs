use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{target::Target, PROJ_DIRS};

#[derive(Debug)]
pub struct TargetList {
    inner: InnerTargetList,
    file: File,
}

impl TargetList {
    pub fn name(&self) -> &str {
        &self.inner.name
    }
}

impl TargetList {
    fn list_dir() -> PathBuf {
        let dirs = PROJ_DIRS.get().expect("directories not initialized");
        let mut path = dirs.data_dir().to_path_buf();
        path.push("lists");
        path
    }

    fn list_file(dir: &Path, name: &str) -> PathBuf {
        let mut path = dir.to_path_buf();
        path.push(name);
        path.set_extension("json");
        path
    }

    pub fn exists(name: &str) -> bool {
        Self::exists_in(name, &Self::list_dir())
    }

    pub fn exists_in(name: &str, directory: &Path) -> bool {
        Self::list_file(directory, name).is_file()
    }

    pub fn new(name: &str, comment: Option<&str>) -> anyhow::Result<Self> {
        let dirs = PROJ_DIRS.get().expect("directories not initialized");

        let dir = Self::list_dir();
        ensure!(
            dir.is_dir() || !dir.exists(),
            "list directory is neither a directory nor nonexistant"
        );

        if !dir.exists() {
            create_dir_all(&dir).context("failed to create list directory")?;
        }

        Self::new_in(name, comment, &dir)
    }

    pub fn new_in(name: &str, comment: Option<&str>, directory: &Path) -> anyhow::Result<Self> {
        ensure!(directory.is_dir(), "directory isn't a directory");

        ensure!(name != "none", "name is \"none\"");
        let name_regex = Regex::new("^[a-z](_?[a-z0-9])+$").expect("correct name regex");
        ensure!(name_regex.is_match(name), "name is forbidden");

        let path = Self::list_file(directory, name);
        ensure!(!path.is_file(), "target list file already exists");
        ensure!(
            !path.exists(),
            "target list file is neither a file nor nonexistent",
        );

        let file = File::create(path).context("failed to create target list file")?;

        let mut ret = Self {
            file,
            inner: InnerTargetList {
                name: name.to_string(),
                comment: comment.map(ToString::to_string),
                targets: Vec::new(),
            },
        };
        ret.save().context("failed to save no target list file")?;

        Ok(ret)
    }

    pub fn load(name: &str) -> anyhow::Result<Self> {
        Self::load_in(name, &Self::list_dir())
    }

    pub fn load_in(name: &str, directory: &Path) -> anyhow::Result<Self> {
        ensure!(directory.is_dir(), "directory isn't a directory");

        let path = Self::list_file(directory, name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .append(false)
            .truncate(false)
            .open(path)
            .context("failed to open target list file")?;

        let inner: InnerTargetList = serde_json::from_reader(BufReader::new(&file))
            .context("failed to parse target list file")?;
        ensure!(inner.name == name, "list name missmatch");

        Ok(Self { inner, file })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        self.file.set_len(0);
        serde_json::to_writer(BufWriter::new(&self.file), &self.inner)
            .context("failed to save target list")
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct InnerTargetList {
    name: String,
    comment: Option<String>,
    targets: Vec<Target>,
}
