use serde::Deserialize;

use super::AsyncQueue;

/// Configuration module used for handling application settings.
///
/// This module provides structures and types for managing application configuration.
///
/// The `Config` struct represents the configuration for the application. It includes the `queue` field of type `AsyncQueue`
/// which holds the configuration for the async queue, and the `interval` field of type `u64` which represents the interval value.
#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    /// The configuration for the async queue.
    pub queue: AsyncQueue,
    /// The interval value indicating how long the queue will wait to process elements again.
    pub interval: u64,
}
