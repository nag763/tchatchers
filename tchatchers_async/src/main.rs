pub mod config;

use tchatchers_core::{
    async_message::{processor::process, AsyncMessage, AsyncQueue},
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
    env_logger::init();
    debug!("Starting async component");
    dotenv::dotenv().ok();
    debug!("Env inited");

    let redis_conn = get_async_pool().await;
    debug!("Redis pool acquired with success");
    let pg_pool = get_pg_pool().await;
    debug!("PG pool acquired with success");

    let config_file = include_str!("conf.yml");
    let queues_config: Vec<Config> = serde_yaml::from_str(config_file).unwrap();
    debug!("Queues successfuly parsed from config file.");

    let mut events: JoinSet<JoinHandle<()>> = JoinSet::new();
    for config in queues_config {
        debug!("The following queue has been parsed from the yaml file : {config:#?}");
        let queue_name = config.queue;
        let redis_conn = redis_conn.clone();
        let pg_pool = pg_pool.clone();
        events.spawn(async move {
            debug!("[{queue_name}] Building queue process...");
            let interval = config.interval;
            debug!("[{queue_name}] Intervall being defined : {interval} (in seconds)");
            let mut interval = time::interval(time::Duration::from_secs(interval));
            loop {
                trace!("[{queue_name}] Ticking clock");
                interval.tick().await;
                debug!("[{queue_name}] Waiting to process events");
                if let Some(events) =
                    AsyncMessage::read_events(queue_name, &mut redis_conn.get().await.unwrap())
                        .await
                {
                    let events_number = events.len();
                    debug!(
                        "[{queue_name}] {events_number} Events found and starting to be processed"
                    );
                    let _ = process(
                        queue_name,
                        events,
                        &pg_pool,
                        &mut redis_conn.get().await.unwrap(),
                    )
                    .await;
                }
                info!("[{queue_name}] Events processed with success, looping again.");
            }
        });

        info!("[{queue_name}] The queue has been built with success.");
    }

    while let Some(handle) = events.join_next().await {
        let _ = handle;
    }

    println!("Done");
}
