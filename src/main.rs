use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context};
use clap::Parser;
use directories::ProjectDirs;

use crate::{state::State, store::DownloadListStore};

mod cli;
mod data;
mod state;
mod store;

fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();
    let cli = cli::CLI::parse();

    let (state_dir, data) = prepare_directories()?;
    let mut state = State::load(&state_dir).context("failed to read state")?;

    #[cfg(debug_assertions)]
    if cli.debug_state {
        dbg!(&state);
    }

    match cli.command {
        cli::Command::List { command } => match command {
            cli::ListCommand::Activate { name } => {
                let _ = DownloadListStore::load(&name, &data)?;
                state.set_active_list(name);
            }
            cli::ListCommand::Create { name, description } => {
                let mut list = DownloadListStore::new(name, &data)?;

                list.set_description(description);

                list.save()?;
            }
            cli::ListCommand::Delete { name } => DownloadListStore::delete(&name, &data)?,
            cli::ListCommand::Info { name } => {
                let list = DownloadListStore::load(&name, &data)?;

                println!("name:        {}", list.name());
                println!("description: {}", list.description());
            }
            cli::ListCommand::List => DownloadListStore::list(&data)?
                .into_iter()
                .for_each(|n| println!("{n}")),
            cli::ListCommand::Update { name, description } => {
                let mut list = DownloadListStore::load(&name, &data)?;

                if let Some(description) = description {
                    list.set_description(description);
                }

                list.save()?;
            }
        },
    }

    #[cfg(debug_assertions)]
    if cli.debug_state {
        dbg!(&state);
    }

    if state.changed() {
        state.save(&state_dir)?;
    }

    Ok(())
}

fn prepare_directories() -> anyhow::Result<(PathBuf, PathBuf)> {
    let dirs = ProjectDirs::from("dev", "TFLD", "listload")
        .context("failed to discover project directories")?;

    let state = dirs.state_dir().unwrap_or_else(|| dirs.data_local_dir());
    prepare_directory(state)?;

    let data = dirs.data_dir();
    prepare_directory(data)?;

    Ok((state.to_path_buf(), data.to_path_buf()))
}

fn prepare_directory(path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        return Ok(());
    }

    ensure!(
        !path.exists(),
        "required directory is not a directory: {}",
        path.display()
    );

    fs::create_dir_all(path).context(format!(
        "failed to create required directory: {}",
        path.display()
    ))
}
