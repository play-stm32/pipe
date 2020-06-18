#![feature(proc_macro_hygiene, decl_macro)]
#![feature(option_result_contains)]
#[macro_use]
extern crate rocket;

mod server;
mod sql_helper;
mod device;
mod user;

use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use crate::server::Server;
use crate::device::token::static_rocket_route_info_for_new_token;
use crate::device::command::static_rocket_route_info_for_send_command;
use crate::device::index::static_rocket_route_info_for_get_register_device;
use crate::user::login::static_rocket_route_info_for_login;

fn main() {
    let server = Server::new().build();
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let clients_clone = clients.clone();
    thread::spawn(move || server.start(clients));

    rocket::ignite()
        .mount("/", routes![new_token,
               send_command,
               login,
               get_register_device])
        .mount("/", StaticFiles::from("static"))
        .manage(clients_clone).launch();
}