use self::{
    env::EnvArgAction, message::MessageArgAction, report::ReportArgs, room::RoomArgAction,
    user::UserArgAction,
};

/// Provides functionality to manage the application's environment.
///
/// This module contains subcommands to manage the current environment, as well as commands to create and
/// populate a new environment file. An environment file is a `.env` file that contains environment
/// variables used by the application. These variables typically include database connection settings,
/// security-related secrets, and other configuration values that are sensitive or subject to change.
pub mod env;
/// Provides functionality to manage the messages stored in the application's database.
///
/// This module contains subcommands to perform CRUD (Create, Read, Update, Delete) operations on the
/// messages stored in the application's database. Messages are the main data entity of the application
/// and contain information about the content of messages, the users who sent them, and the rooms where
/// they were sent. The commands in this module allow users to create new messages, retrieve messages
/// from the database, update existing messages, and delete messages from the database.
pub mod message;
/// Provides functionality to manage the rooms in the application.
///
/// This module contains subcommands to perform operations on the rooms in the application. Rooms are
/// entities that store messages sent by users. The commands in this module allow users to retrieve
/// messages from a room, delete all messages from a room, and view the global activity of all rooms
pub mod room;
/// Provides functionality to manage the users of the application.
///
/// This module contains subcommands to perform CRUD (Create, Read, Update, Delete) operations on the
/// users of the application. Users are entities that represent individual users of the application,
/// and contain information such as usernames, passwords, and email addresses. The commands in this
/// module allow users to create new users, retrieve information about existing users, update user
/// information, and delete users from the application.
pub mod user;

pub mod report;

/// The CLI arguments that will be parsed from the user input.
#[derive(clap::Parser, Debug)]
#[command(
    author = "LABEYE Loïc",
    version,
    about = "tct is a CLI tool designed to run the common operations on the tchatchers_app",
    after_help = "This tool is meant to be used within the tchatchers project.\nRun it only either within the workspace (dev) or as a binary (prod)."
)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub entity: CliEntityArg,
    #[clap(long, short, value_hint = clap::ValueHint::FilePath, required=false, global=true, help="Precise a .env file.")]
    pub env: Option<std::path::PathBuf>,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum CliEntityArg {
    #[command(about = "Manages the users of the application")]
    User {
        #[command(subcommand)]
        action: UserArgAction,
    },
    #[command(about = "Manages the rooms of the application")]
    Room {
        #[command(subcommand)]
        action: RoomArgAction,
    },
    #[command(about = "Manages the messages stored in the database")]
    Message {
        #[command(subcommand)]
        action: MessageArgAction,
    },
    #[command(about = "Helper to either set up a new environment or check the current one")]
    Env {
        #[command(subcommand)]
        action: EnvArgAction,
    },
    #[command(about = "Menu to check the reports made by the users")]
    Report {
        #[command(subcommand)]
        action: ReportArgs,
    },
}
