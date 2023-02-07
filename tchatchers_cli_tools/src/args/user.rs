use crate::common::user::{UserIdentifier, UserSearch};

#[derive(Debug, Clone, clap::Subcommand)]
pub enum UserArgAction {
    Create,
    Update {
        id: i32,
    },
    Deactivate {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    Activate {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    Delete {
        #[command(subcommand)]
        user_identifier: UserIdentifier,
    },
    Search {
        #[command(subcommand)]
        user_search: UserSearch,
    },
}
