#[derive(clap::Subcommand, Debug, Clone)]
pub enum UserIdentifier {
    Id { value: i32 },
    Login { value: String },
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum UserSearch {
    Id {value : i32},
    Login { value: String},
    Name {value: String}
}