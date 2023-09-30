use clap::{Parser, Subcommand};

mod list;

pub use list::*;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
    /// Print the state before and after executing the specified operation.
    #[cfg(debug_assertions)]
    #[clap(long)]
    pub debug_state: bool,
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
