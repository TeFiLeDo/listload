use std::sync::OnceLock;

use anyhow::Context;
use clap::{Parser, Subcommand};
use config::Config;
use directories::{ProjectDirs, UserDirs};

mod config;

static USER_DIRS: OnceLock<UserDirs> = OnceLock::new();
static PROJ_DIRS: OnceLock<ProjectDirs> = OnceLock::new();

fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    // initialize dirs
    let user_dirs = UserDirs::new().context("failed to discover user directiories")?;
    let proj_dirs = ProjectDirs::from("dev", "TFLD", "ListLoad")
        .context("failed to discover program directories")?;

    USER_DIRS
        .set(user_dirs)
        .ok()
        .context("failed to initialize user directories")?;
    PROJ_DIRS
        .set(proj_dirs)
        .ok()
        .context("failed to initialize program directories")?;

    if let CMD::License = cli.command {
        println!("{}", include_str!("../LICENSE"));
        return Ok(());
    }

    // prepare for operation
    let cfg = Config::read_from_default_file().context("failed to load config")?;
    let downloader = cfg.downloader().context("failed to create downloader")?;

    match cli.command {
        CMD::Config => {
            println!("{cfg}");
            Ok(())
        }
        CMD::License => {
            panic!("license passed first check");
        }
    }
}

#[derive(Parser)]
#[clap(about, author, version)]
struct CLI {
    #[clap(subcommand)]
    command: CMD,
}

#[derive(Subcommand)]
enum CMD {
    /// Print out the current configuration.
    Config,
    /// Print the EUPL 1.2, under which this program is licensed.
    License,
}
