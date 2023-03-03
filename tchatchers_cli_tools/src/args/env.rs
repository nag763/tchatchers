use std::ffi::OsString;

/// The actions that can be run on the environment variables.
#[derive(clap::Subcommand, Debug, Clone)]
pub enum EnvArgAction {
    /// Dialog to create a new environment (can erase the current one)
    #[command(about = "Dialog to create a new environment (can erase the current one)")]
    Create,
    /// Check the current environment
    #[command(about = "Check the current environment")]
    Check,
    /// Build a parameterized Nginx configuration file.
    #[command(about = "Build a parameterized Nginx configuration")]
    BuildNginxConf {
        #[arg(short = 'o', default_value = "nginx.conf")]
        output_file: OsString,
    },
}
