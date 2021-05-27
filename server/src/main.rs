#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]
#![warn(missing_docs)]

//! Logs collector and seeker. Kind of.

mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let handlers_engine = web::Data::new(handlers::make_handlers_engine());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(handlers_engine.clone())
            .service(handlers::jsonrpc_handler)
    });
    server.bind("localhost:8080")?.run().await
}
