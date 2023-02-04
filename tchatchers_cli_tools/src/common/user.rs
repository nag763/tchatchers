#[derive(clap::Subcommand, Debug, Clone)]
pub enum UserIdentifier {
    Id { value: i32 },
    Login { value: String },
}
