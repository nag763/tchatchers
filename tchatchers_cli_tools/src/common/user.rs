/// The user identifier is a struct used to query and return a single result from the user DB.
///
/// It can only be used to get a single result from the database.
#[derive(clap::Subcommand, Debug, Clone, Display)]
pub enum UserIdentifier {
    /// Performing a search on the ID.
    #[command(about = "Unique identifier of the user")]
    Id { value: i32 },
    /// Performing a search on the login.
    #[command(about = "User's login")]
    Login { value: String },
    /// Performing a search on the  mail.
    #[command(about = "User's email")]
    Email { value: String },
}

/// The user search struct is a struct used to return possibly several results from the user DB.
#[derive(clap::Subcommand, Debug, Clone, Display)]
pub enum UserSearch {
    /// Performing a search on the ID.
    #[command(about = "Unique identifier of the user")]
    Id { value: i32 },
    /// Performing a search on the login.
    #[command(about = "User's login")]
    Login { value: String },
    /// Performing a search on the  mail.
    #[command(about = "User's email")]
    Email { value: String },
    /// Performing a search on the name (can return several results).
    #[command(about = "The user name (might affect or return several users in consequence)")]
    Name { value: String },
}
