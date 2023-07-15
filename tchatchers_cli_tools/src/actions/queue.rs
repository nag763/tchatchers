use tchatchers_core::{
    async_message::{processor::process, AsyncQueue, QueueReport},
    pool,
};
use tokio::task::JoinSet;

use crate::errors::CliError;

pub struct QueueArgAction;

impl QueueArgAction {
    pub async fn get_queue_report(
        queue: Option<AsyncQueue>,
        limit: Option<i64>,
    ) -> Result<(), CliError> {
        let pg_pool = pool::get_pg_pool().await;

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

    pub async fn delete_events(queue: AsyncQueue, list: Vec<String>) {
        let number_of_elements_to_delete = list.len();
        let redis_pool = pool::get_async_pool().await;
        let mut redis_conn = redis_pool.get().await.unwrap();
        let number_of_events_deleted = queue.delete(list, &mut redis_conn).await;
        match number_of_events_deleted {
            v if v == number_of_elements_to_delete => (),
            0 => println!("No events were deleted"),
            v => println!("The number of elements deleted {v} doesn't match the number of requested elements to delete {number_of_elements_to_delete}"),
        };
    }

    pub async fn read_events(queue: AsyncQueue) {
        let redis_pool = pool::get_async_pool().await;
        let mut redis_conn = redis_pool.get().await.unwrap();
        if let Some(events) = queue.read_events_with_timeout(&mut redis_conn).await {
            print!("{events:#?}");
        } else {
            println!("[{queue}] No event founds to process");
        }
    }

    pub async fn clear(queue: AsyncQueue) {
        let redis_pool = pool::get_async_pool().await;
        let mut redis_conn = redis_pool.get().await.unwrap();
        let number_of_events_cleared = queue.clear_with_timeout(&mut redis_conn).await;
        println!("[{queue}] {number_of_events_cleared} events cleared");
    }

    pub async fn process(queue: AsyncQueue) {
        let (redis_pool, pg_pool) = (pool::get_async_pool().await, pool::get_pg_pool().await);
        let mut redis_conn = redis_pool.get().await.unwrap();
        if let Some(events) = queue.read_events_with_timeout(&mut redis_conn).await {
            let events_number = events.len();
            println!("[{queue}] {events_number} events found");
            let _ = process(queue, events, &pg_pool, &mut redis_conn).await;
            println!("[{queue}] Events processed with success");
        } else {
            println!("[{queue}] No event founds to process");
        }
    }
}
