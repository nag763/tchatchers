use tchatchers_core::async_message::AsyncQueue;

/// The actions that can be run on the messages stored in the database.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum QueueArg {
    /// Clear the events of a queue.
    #[clap(about = "Clear the events of a queue")]
    Clear {
        /// The queue to clear.
        #[clap(name = "queue")]
        queue: AsyncQueue,
    },
    /// Process directly all the events of a queue.
    #[clap(about = "Process directly all the events of a queue")]
    Process {
        /// The queue to process.
        #[clap(name = "queue")]
        queue: AsyncQueue,
    },
    /// Read all the events of a queue.
    #[clap(about = "Read all the events of a queue")]
    ReadEvents {
        /// The queue to read events from.
        #[clap(name = "queue")]
        queue: AsyncQueue,
    },
    /// Delete directly some events.
    #[clap(about = "Delete directly some events")]
    DeleteEvents {
        /// The queue to delete events from.
        #[clap(name = "queue")]
        queue: AsyncQueue,
        /// The events to delete.
        #[clap(name = "events", required = true)]
        events: Vec<String>,
    },
    /// Get report of latest executed processes for a queue.
    #[clap(about = "Get report of latest executed processes for a queue")]
    Report {
        /// An optional queue to retrieve the report for. If not specified, reports for all queues will be retrieved.
        #[clap(name = "queue")]
        queue: Option<AsyncQueue>,
        /// The maximum number of elements to display in the report.
        #[clap(
            name = "number",
            short = 'n',
            long = "number",
            help = "Number of events to show"
        )]
        number: Option<i64>,
    },
}
