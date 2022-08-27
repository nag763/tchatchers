pub mod ws;

use actix_web::{web, App, HttpServer};
use ws::ws_index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(ws_index))))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
