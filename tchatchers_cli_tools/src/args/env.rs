#[derive(clap::ValueEnum, Debug, Clone)]
pub enum EnvArgAction {
    Create,
    Check,
}
