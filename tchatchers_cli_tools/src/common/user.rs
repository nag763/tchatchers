#[derive(clap::Subcommand, Debug, Clone)]
pub enum UserIdentifier {
    #[command(about="Unique identifier of the user")]
    Id { value: i32 },
    #[command(about="User's login")]
    Login { value: String },
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum UserSearch {
    #[command(about="Unique identifier of the user")]
    Id { value: i32 },
    #[command(about="User's login")]
    Login { value: String },
    #[command(about="The user name (might affect or return several users in consequence)")]
    Name { value: String },
}
