use crate::common::output::OutputStream;

/// The actions that can be run on the environment variables.
#[derive(clap::Subcommand, Debug, Clone)]
pub enum EnvArgAction {
    /// Dialog to create a new environment (can erase the current one)
    #[command(about = "Dialog to create a new environment (can erase the current one)")]
    Create(OutputStream),
    /// Check the current environment
    #[command(about = "Check the current environment")]
    Check,
    /// Build a parameterized Nginx configuration file.
    #[command(about = "Build a parameterized Nginx configuration")]
    BuildNginxConf(OutputStream),
}
