use crate::common::user::{UserIdentifier, UserSearch};

/// Enumeration of the actions that can be performed on users.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum UserArgAction {
    /// Interactive dialog to create a new user.
    #[command(about = "Interactive dialog to create a new user")]
    Create,
    /// Deactivates the user passed as argument.
    #[command(about = "Deactivates the user passed as argument")]
    Deactivate {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    /// Activates the user passed as argument.
    #[command(about = "Activates the user passed as argument")]
    Activate {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    /// Delete the user passed as argument.
    #[command(about = "Delete the user passed as argument")]
    Delete {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    /// Tool to search one user.
    #[command(about = "Tool to search one user")]
    Search {
        #[command(subcommand)]
        user_search: UserSearch,
    },
}
