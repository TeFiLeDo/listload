use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Seek},
    ops::{Deref, DerefMut},
    path::Path,
};

use anyhow::{ensure, Context};
use fs4::FileExt;

use crate::data::DownloadList;

#[derive(Debug)]
pub struct DownloadListStore {
    list: DownloadList,
    file: File,
}

impl DownloadListStore {
    pub fn new(name: String, directory: &Path) -> anyhow::Result<Self> {
        let path = Self::path_in_directory(directory, &name);

        let file = OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(&path)
            .context("failed to create list file")?;

        file.lock_exclusive()
            .context("failed to acquire list file lock")?;

        Ok(Self {
            list: DownloadList::new(name),
            file,
        })
    }

    pub fn load(name: &str, directory: &Path) -> anyhow::Result<Self> {
        let path = Self::path_in_directory(directory, name);

        let file = OpenOptions::new()
            .create(false)
            .read(true)
            .write(true)
            .open(&path)
            .context("failed to open list file")?;

        file.lock_exclusive()
            .context("failed to acquire list file lock")?;

        let reader = BufReader::new(&file);
        let list: DownloadList =
            serde_json::from_reader(reader).context("failed to deserialize list")?;

        ensure!(
            name == list.name(),
            "list name mismatch: found {name} instead of {}",
            list.name()
        );

        Ok(Self { list, file })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        self.file.set_len(0).context("failed to clear list file")?;
        self.file
            .seek(std::io::SeekFrom::Start(0))
            .context("failed to find list file start")?;

        let writer = BufWriter::new(&self.file);

        if cfg!(debug_assertions) {
            serde_json::to_writer_pretty(writer, &self.list)
        } else {
            serde_json::to_writer(writer, &self.list)
        }
        .context("failed to write list file")
    }

    pub fn delete(name: &str, directory: &Path) -> anyhow::Result<()> {
        fs::remove_file(Self::path_in_directory(directory, name))
            .context("failed to remove list file")
    }

    fn path_in_directory(directory: &Path, name: &str) -> std::path::PathBuf {
        directory.join(name).with_extension("json")
    }
}

impl Deref for DownloadListStore {
    type Target = DownloadList;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl DerefMut for DownloadListStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}
