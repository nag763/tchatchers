use tchatchers_core::{
    async_message::{processor::process, AsyncQueue, QueueReport},
    pool,
};
use tokio::task::JoinSet;

use crate::errors::CliError;

/// A struct representing actions related to queue management.
pub struct QueueArgAction;

impl QueueArgAction {
    /// Retrieves the queue report for the specified queue.
    ///
    /// Retrieves the latest queue report for the given queue from the database
    /// and prints the report to the console. The `limit` parameter can be used
    /// to limit the number of reports returned.
    ///
    /// # Arguments
    ///
    /// - `queue`: An optional `AsyncQueue` representing the queue to retrieve the report for.
    ///   If `None`, the report for all queues will be retrieved.
    /// - `limit`: An optional `i64` representing the maximum number of reports to return.
    ///
    /// Returns an `Ok` result if the operation succeeds, or a `CliError` if an error occurs.
    pub async fn get_queue_report(
        queue: Option<AsyncQueue>,
        limit: Option<i64>,
    ) -> Result<(), CliError> {
        if let Some(limit) = limit {
            if limit < 1 {
                println!("Please provide a positive limit");
                return Ok(());
            }
        }
        let pg_pool = pool::get_pg_pool().await?;

        let reports = match queue {
            Some(queue) => QueueReport::latest_for_queue(queue, limit, &pg_pool).await?,
            None => {
                let mut reports: Vec<QueueReport> = vec![];
                let mut task: JoinSet<Result<Vec<QueueReport>, sqlx::error::Error>> =
                    JoinSet::new();
                for queue in AsyncQueue::iter() {
                    let conn = pg_pool.clone();
                    task.spawn(
                        async move { QueueReport::latest_for_queue(queue, limit, &conn).await },
                    );
                }
                while let Some(records) = task.join_next().await {
                    if let Ok(record_joined) = records {
                        reports.append(&mut record_joined?);
                    };
                }
                reports
            }
        };
        if reports.is_empty() {
            println!("No report found for request");
        } else {
            reports.iter().for_each(|r| println!("{r}"));
        }

        Ok(())
    }

    /// Deletes events from the specified queue.
    ///
    /// Deletes the specified events from the given queue in Redis. The number of elements
    /// deleted is printed to the console.
    ///
    /// # Arguments
    ///
    /// - `queue`: An `AsyncQueue` representing the queue to delete events from.
    /// - `list`: A vector of strings representing the events to delete.
    pub async fn delete_events(queue: AsyncQueue, list: Vec<String>) -> Result<(), CliError> {
        let number_of_elements_to_delete = list.len();
        let redis_pool = pool::get_async_pool().await?;
        let mut redis_conn = redis_pool.get().await.unwrap();
        let number_of_events_deleted = queue.delete(list, &mut redis_conn).await;
        match number_of_events_deleted {
            v if v == number_of_elements_to_delete => (),
            0 => println!("No events were deleted"),
            v => println!("The number of elements deleted {v} doesn't match the number of requested elements to delete {number_of_elements_to_delete}"),
        };
        Ok(())
    }

    /// Reads events from the specified queue.
    ///
    /// Reads events from the given queue in Redis and prints them to the console.
    ///
    /// # Arguments
    ///
    /// - `queue`: An `AsyncQueue` representing the queue to read events from.
    pub async fn read_events(queue: AsyncQueue) -> Result<(), CliError> {
        let redis_pool = pool::get_async_pool().await?;
        let mut redis_conn = redis_pool.get().await.unwrap();
        if let Some(events) = queue.read_events_with_timeout(&mut redis_conn).await {
            print!("{events:#?}");
        } else {
            println!("[{queue}] No event founds to process");
        }
        Ok(())
    }

    /// Clears events from the specified queue.
    ///
    /// Clears all events from the given queue in Redis. The number of events cleared is
    /// printed to the console.
    ///
    /// # Arguments
    ///
    /// - `queue`: An `AsyncQueue` representing the queue to clear.
    pub async fn clear(queue: AsyncQueue) -> Result<(), CliError> {
        let redis_pool = pool::get_async_pool().await?;
        let mut redis_conn = redis_pool.get().await.unwrap();
        let number_of_events_cleared = queue.clear_with_timeout(&mut redis_conn).await;
        println!("[{queue}] {number_of_events_cleared} events cleared");
        Ok(())
    }

    /// Processes events from the specified queue.
    ///
    /// Reads events from the given queue in Redis, processes them, and prints the result
    /// to the console. The events are processed by running the `process` function from
    /// the `async_message` module.
    ///
    /// # Arguments
    ///
    /// - `queue`: An `AsyncQueue` representing the queue to process events from.
    pub async fn process(queue: AsyncQueue) -> Result<(), CliError> {
        let (redis_pool, pg_pool) = (pool::get_async_pool().await?, pool::get_pg_pool().await?);
        let mut redis_conn = redis_pool.get().await.unwrap();
        if let Some(events) = queue.read_events_with_timeout(&mut redis_conn).await {
            let events_number = events.len();
            println!("[{queue}] {events_number} events found");
            let _ = process(queue, events, &pg_pool, &mut redis_conn).await;
            println!("[{queue}] Events processed with success");
        } else {
            println!("[{queue}] No event founds to process");
        }
        Ok(())
    }
}
