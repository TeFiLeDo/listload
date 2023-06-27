use std::{path::PathBuf, sync::OnceLock};

use anyhow::{ensure, Context};
use clap::{Parser, Subcommand};
use config::Config;
use directories::{ProjectDirs, UserDirs};
use persistent_state::PersistentState;
use target::Target;
use target_list::TargetList;
use url::Url;

mod config;
mod persistent_state;
mod target;
mod target_list;

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
    let mut persistent =
        PersistentState::read_from_default_file().context("failed to load persistent state")?;

    match cli.command {
        CMD::Config => {
            println!("{cfg}");
        }
        CMD::PersistentState => {
            println!("{persistent}");
        }
        CMD::List { cmd } => match cmd {
            ListCommand::Create {
                name,
                keep_current_selected: keep_current_active,
                comment,
            } => {
                if TargetList::exists(&name) {
                    eprintln!("list already exists");
                } else {
                    TargetList::new(&name, comment.as_ref().map(|c| c.as_str()))
                        .context("failed to create target list")?;
                }

                if !keep_current_active {
                    persistent.set_list(&name);
                }
            }
            ListCommand::Select { name } => {
                if name == "none" {
                    persistent.clear_list();
                } else if TargetList::exists(&name) {
                    persistent.set_list(&name);
                } else {
                    eprintln!("list doesn't exist");
                }
            }
        },
        CMD::Target { cmd } => {
            let list = persistent.list().context("no list selected")?;
            let mut list = TargetList::load(&list).context("failed to load list")?;

            match cmd {
                TargetCommand::Create {
                    file,
                    url,
                    comment,
                    keep_current_selected,
                } => {
                    let target = Target::new(url, &file, comment.as_ref().map(|c| c.as_str()))
                        .context("invalid target")?;
                    list.add_target(target);

                    if !keep_current_selected {
                        persistent.set_target(list.len_targets() - 1);
                    }
                }
                TargetCommand::Select { index } => {
                    if index < list.len_targets() {
                        persistent.set_target(index);
                    } else {
                        eprintln!("target doesn't exist");
                    }
                }
            }

            list.save().context("failed to save list")?;
        }
        CMD::License => {
            panic!("late command");
        }
    }

    persistent.save_to_default_file()
}

#[derive(Parser)]
#[clap(about, author, version)]
struct CLI {
    #[clap(subcommand)]
    command: CMD,
}

#[derive(Subcommand)]
enum CMD {
    /// Print the current configuration.
    Config,
    /// Print the EUPL 1.2, under which this program is licensed.
    License,
    /// Print the current persistent state.
    PersistentState,
    /// Target list operations.
    List {
        #[clap(subcommand)]
        cmd: ListCommand,
    },
    /// Individual target operations.
    Target {
        #[clap(subcommand)]
        cmd: TargetCommand,
    },
}

#[derive(Subcommand)]
enum ListCommand {
    /// Create a new list.
    Create {
        /// The new lists name.
        ///
        /// The name must start with a lowercase letter (`a-z`). After that, it consists of at least
        /// one lowercase letter (`a-z`) or number (`0-9`). It may also contain nonconsecutive
        /// underscores (`_`), but must not end with one. The name must not be `none`.
        ///
        /// Valid examples: default, version15, my_4_funny_pictures
        ///
        /// Invalid examples: none, 14, _hi, hi_, h__i
        name: String,
        /// A comment to remember what the list is meant to do.
        #[clap(long, short)]
        comment: Option<String>,
        /// Don't select the newly created list.
        #[clap(long, short)]
        keep_current_selected: bool,
    },
    /// Select an existing list.
    ///
    /// List selection is important for the `target` subcommand.
    Select {
        /// The name of the list.
        ///
        /// The special value `none` deselects all lists.
        #[clap(group = "target")]
        name: String,
    },
}

#[derive(Subcommand)]
enum TargetCommand {
    /// Create a new target.
    Create {
        /// The local file name.
        file: PathBuf,
        /// A list of URLs the file is available at.
        url: Vec<Url>,
        /// A comment to remember why the target is in the list.
        #[clap(long, short)]
        comment: Option<String>,
        /// Don't select the newly created target.
        #[clap(long, short)]
        keep_current_selected: bool,
    },
    /// Select an existing target.
    /// Target selection is important for the `url` subcommand.
    Select {
        /// The index of the target.
        index: usize,
    },
}
