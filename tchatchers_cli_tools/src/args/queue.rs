use tchatchers_core::async_message::AsyncQueue;

/// The actions that can be run on the messages stored in the database.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum QueueArg {
    #[command(about = "Clear the events of a queue")]
    Clear { queue: AsyncQueue },
    #[command(about = "Process directly all the events of a queue")]
    Process { queue: AsyncQueue },
    #[command(about = "Read all the events of a queue")]
    ReadEvents { queue: AsyncQueue },
    #[command(about = "Delete directly some events")]
    DeleteEvents {
        queue: AsyncQueue,
        #[arg(required = true)]
        events: Vec<String>,
    },
    #[command(about = "Get report of latest executed processes for a queue")]
    Report {
        queue: Option<AsyncQueue>,
        #[arg(help = "Number of elements to display", short = 'l', long = "limit")]
        limit: Option<i64>,
    },
}
