use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn get_pool() -> PgPool {
    //    let connection_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(15)
        .connect("postgres://chatter:6@l6QFn5eP56eHA^Ki%IQgaj5BTw7cGd1@postgreshost/chatapp")
        .await
        .unwrap()
}
