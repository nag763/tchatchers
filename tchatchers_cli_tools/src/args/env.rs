/// The actions that can be run on the environment variables.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum EnvArgAction {
    /// Dialog to create a new environment (can erase the current one)
    #[value(help = "Dialog to create a new environment (can erase the current one)")]
    Create,
    /// Check the current environment
    #[value(help = "Check the current environment")]
    Check,
}
