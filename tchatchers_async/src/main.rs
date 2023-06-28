use tchatchers_core::{
    async_message::{AsyncMessage, AsyncQueues},
    pool::get_redis_async_pool,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let redis_conn = get_redis_async_pool();

    let _ = tokio::spawn(async move {
        let mut conn = redis_conn.get().unwrap();
        loop {
            println!("Waiting to process events");
            while let Some(events) =  AsyncMessage::read_events(AsyncQueues::LoggedUsers, &mut conn) {
                println!("{events:?}");
            }
            println!("Done");
        }
    });

    println!("Done");
}
