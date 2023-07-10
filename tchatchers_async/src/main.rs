use tchatchers_core::{
    async_message::{processor::process, AsyncMessage, AsyncQueue},
    pool::{get_async_pool, get_pg_pool},
};
use tokio::{task, time};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Starting async component");
    dotenv::dotenv().ok();
    debug!("Env inited");

    let redis_conn = get_async_pool().await;
    debug!("Redis pool acquired with success");
    let pg_pool = get_pg_pool().await;
    debug!("PG pool acquired with success");
    let logg_async = task::spawn(async move {
        debug!("Building log in pool.");
        let mut interval = time::interval(time::Duration::from_secs(60));
        loop {
            trace!("Ticking clock");
            interval.tick().await;
            info!("Waiting to process events");
            if let Some(events) = AsyncMessage::read_events(
                AsyncQueue::LoggedUsers,
                &mut redis_conn.get().await.unwrap(),
            )
            .await
            {
                let _ = process(
                    AsyncQueue::LoggedUsers,
                    events,
                    &pg_pool,
                    &mut redis_conn.get().await.unwrap(),
                )
                .await;
            }
            info!("Done");
        }
    });

    let _ = logg_async.await;

    println!("Done");
}
