use crate::common::user::{UserIdentifier, UserSearch};

#[derive(Debug, Clone, clap::Subcommand)]
pub enum UserArgAction {
    #[command(about="Interactive dialog to create a new user")]
    Create,
    #[command(about="Deactivates the user passed as argument")]
    Deactivate {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    #[command(about="Activates the user passed as argument")]
    Activate {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    #[command(about="Delete the user passed as argument")]
    Delete {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    #[command(about="Tool to search one user")]
    Search {
        #[command(subcommand)]
        user_search: UserSearch,
    },
}
