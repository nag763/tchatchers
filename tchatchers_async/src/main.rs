//! Main entry point for the async component of the application.
//!
//! This module contains the main function that initializes and runs the async component of the application.
//! It sets up the necessary connections, reads the configuration, and processes the queues in separate tasks.

//! This module provides configuration related functionality.
//!
//! It includes structs and functions for reading and parsing configuration files.
pub mod config;

use tchatchers_core::{
    async_message::{processor::process, AsyncQueue},
    pool::{get_async_pool, get_pg_pool},
};
use tokio::{
    task::{JoinHandle, JoinSet},
    time,
};

use crate::config::Config;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();
    debug!("Starting async component");

    // Load environment variables from .env file
    dotenv::dotenv().ok();
    debug!("Env initialized");

    // Acquire the Redis connection pool
    let redis_conn = get_async_pool().await;
    debug!("Redis pool acquired with success");

    // Acquire the PostgreSQL connection pool
    let pg_pool = get_pg_pool().await;
    debug!("PG pool acquired with success");

    // Read the configuration file
    let config_file = include_str!("conf.yml");
    let queues_config: Vec<Config> = serde_yaml::from_str(config_file).unwrap();
    debug!("Queues successfully parsed from config file.");

    // Create a set of join handles for tracking spawned tasks
    let mut events: JoinSet<JoinHandle<()>> = JoinSet::new();

    // Process each queue in separate tasks
    for config in queues_config {
        let queue_name = config.queue.clone();
        let redis_conn = redis_conn.clone();
        let pg_pool = pg_pool.clone();

        // Spawn a new task for processing the queue
        events.spawn(async move {
            debug!("[{}] Building queue process...", queue_name);
            let interval = config.interval;
            debug!(
                "[{}] Interval being defined: {} (in seconds)",
                queue_name, interval
            );

            let mut interval = time::interval(time::Duration::from_secs(interval));

            loop {
                trace!("[{}] Ticking clock", queue_name);
                interval.tick().await;
                debug!("[{}] Waiting to process events", queue_name);

                // Read events from the queue
                if let Some(events) = queue_name
                    .read_events(&mut redis_conn.get().await.unwrap())
                    .await
                {
                    let events_number = events.len();
                    debug!(
                        "[{}] {} Events found and starting to be processed",
                        queue_name, events_number
                    );

                    // Process the events
                    let _ = process(
                        queue_name,
                        events,
                        &pg_pool,
                        &mut redis_conn.get().await.unwrap(),
                    )
                    .await;
                }

                info!(
                    "[{}] Events processed with success, looping again.",
                    queue_name
                );
            }
        });

        info!("[{}] The queue has been built with success.", queue_name);
    }

    // Wait for all tasks to complete
    while let Some(handle) = events.join_next().await {
        let _ = handle;
    }

    println!("Done");
}
