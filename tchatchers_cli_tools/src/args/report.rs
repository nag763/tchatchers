/// An enum representing different subcommands related to reporting.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum ReportArgs {
    /// Subcommand to check the latest reports made by the users.
    #[command(about = "Check the latest reports made by the users")]
    Check,
}
