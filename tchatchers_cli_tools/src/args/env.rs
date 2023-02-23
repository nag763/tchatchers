/// The actions that can be ran on the environment variables.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum EnvArgAction {
    #[value(help = "Dialog to create a new environment (can erase the current one)")]
    Create,
    #[value(help = "Check the current environment")]
    Check,
}
