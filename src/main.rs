#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod server;
mod device;
mod user;
mod db;
mod schema;

use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;

use crate::server::Server;
use crate::device::token::static_rocket_route_info_for_new_token;
use crate::device::command::static_rocket_route_info_for_send_command;
use crate::device::index::static_rocket_route_info_for_get_register_device;
use crate::user::login::static_rocket_route_info_for_login;
use crate::db::token::static_rocket_route_info_for_token_read;
use crate::db::token::static_rocket_route_info_for_token_read_by_value;
use crate::db::token::static_rocket_route_info_for_token_read_by_owner;
use crate::db::token::static_rocket_route_info_for_token_delete;
use crate::db::token::static_rocket_route_info_for_token_create;
use crate::db::user::static_rocket_route_info_for_user_read;
use crate::db::user::static_rocket_route_info_for_user_read_by_name;

#[database("info")]
pub struct DbConn(diesel::MysqlConnection);

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", StaticFiles::from("static"))
        .mount("/device", routes![new_token, send_command, get_register_device])
        .mount("/user", routes![login])
        .mount("/db/token", routes![token_read, token_read_by_value, token_read_by_owner, token_delete, token_create])
        .mount("/db/user", routes![user_read, user_read_by_name])
}

fn main() {
    let server = Server::new().build();
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let clients_clone = clients.clone();
    thread::spawn(move || server.start(clients));
    rocket().manage(clients_clone).launch();
}