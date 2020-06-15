mod server;
mod sql_helper;
mod device;
mod user;

use actix_web::{web, App, HttpServer};
use std::thread;
use crate::server::Server;
use crate::device::token::get_new_token;
use crate::device::command::send_command;
use std::sync::Mutex;
use std::collections::HashMap;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new().build();
    let data = Mutex::new(HashMap::new());
    let data = web::Data::new(data);
    let clients = data.clone().into_inner();
    thread::spawn(move || server.start(clients));

    HttpServer::new(move || {
        App::new().service(
            web::scope("/device")
                .service(get_new_token)
                .app_data(data.clone())
                .service(send_command)
        )
    }).bind("127.0.0.1:8000")?.run().await
}