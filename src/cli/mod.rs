use clap::{Parser, Subcommand};

mod list;

pub use list::*;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage entire download lists.
    #[clap(visible_alias = "l")]
    List {
        #[clap(subcommand)]
        command: ListCommand,
    },
}
