mod server;
mod sql_helper;
mod device;

use actix_web::{web, App, HttpServer};
use std::thread;
use crate::server::Server;
use crate::device::token::get_new_token;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new().build();
    thread::spawn(move || server.start());

    HttpServer::new(|| {
        App::new().service(
            web::scope("/device").
                service(get_new_token)
        )
    }).bind("127.0.0.1:8000")?.run().await
}