use tchatchers_core::{
    async_message::{processor::process, AsyncMessage, AsyncQueue},
    pool::{self, get_async_pool, get_pg_pool},
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

    let redis_conn = get_async_pool();
    let pg_pool = get_pg_pool().await;
    debug!("Redis pool acquired with success");
    let logg_async = task::spawn(async move {
        debug!("Building log in pool.");
        let mut conn = redis_conn.get().unwrap();
        let mut interval = time::interval(time::Duration::from_secs(60));
        loop {
            debug!("Ticking clock");
            interval.tick().await;
            debug!("Waiting for next");
            info!("Waiting to process events");
            if let Some(events) = AsyncMessage::read_events(AsyncQueue::LoggedUsers, &mut conn) {
                process(AsyncQueue::LoggedUsers, events, pg_pool.clone());
            }
            info!("Done");
        }
    });

    let _ = logg_async.await;

    println!("Done");
}
