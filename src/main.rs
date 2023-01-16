use actix_web::{middleware, App, HttpServer};
use std::{env, io};

mod controllers;

use controllers::task;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(task::list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
