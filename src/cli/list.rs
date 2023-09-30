use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ListCommand {
    /// Create a new download list.
    #[clap(visible_alias = "c", visible_alias = "new")]
    Create {
        /// A unique name, which will be used to refer to the list.
        name: String,
        /// A short description of the lists purpose.
        #[clap(long, short, default_value = "", hide_default_value = true)]
        description: String,
    },
    /// Delete a download list.
    #[clap(visible_alias = "d", visible_alias = "rm")]
    Delete {
        /// The name of the list to remove.
        name: String,
    },
    /// Print a download lists properties.
    ///
    #[clap(visible_alias = "i", visible_alias = "show")]
    Info {
        /// The name of the list to inspect.
        name: String,
    },
    /// List all known download lists.
    #[clap(visible_alias = "l", visible_alias = "ls")]
    List,
    /// Update an existing download list.
    ///
    /// Only values specified for this command are changed.
    #[clap(visible_alias = "u")]
    Update {
        /// The name of the list to change.
        name: String,
        /// A short description of the lists purpose.
        ///
        /// Set this to an empty string (e.g. pass "" as value) to remove the current description.
        #[clap(long, short)]
        description: Option<String>,
    },
}
