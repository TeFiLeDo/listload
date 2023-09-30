use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ListCommand {
    /// Create a new download list.
    #[clap(visible_alias = "new", visible_alias = "c")]
    Create {
        /// A unique name, which will be used to refer to the list.
        name: String,
        /// A short description of the lists purpose.
        #[clap(long, short, default_value = "", hide_default_value = true)]
        description: String,
    },
    /// Delete a download list.
    #[clap(visible_alias = "rm", visible_alias = "d")]
    Delete {
        /// The name of the list to remove.
        name: String,
    },
    /// Update an existing list.
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
