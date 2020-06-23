use protocol::protocol::Command;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::Write;
use rocket::State;
use rocket_contrib::json::Json;
use rocket::http::Cookies;
use crate::cookie::check_status;

#[post("/<uuid>", format = "json", data = "<command>")]
pub fn send_command(uuid: String, command: Json<Command>
                    , clients: State<Arc<Mutex<HashMap<String, TcpStream>>>>) -> String {
    let client = clients.lock().unwrap();

    if let Some(mut client) = client.get(&uuid) {
        client.write(&serde_json::to_vec(&command.into_inner()).unwrap()).unwrap();
        "Ok".to_string()
    } else {
        "No Match Device".to_string()
    }
}

#[get("/get_all_commands")]
pub fn get_all_commands(cookies: Cookies<'_>) -> String {
    let status = check_status(&cookies);
    let pass = status.0;

    let mut command_string = String::default();
    if pass {
        let commands = Command::get_field_names();
        for command in commands {
            command_string.insert_str(command_string.len(), command);
            command_string.insert(command_string.len(), ',');
        }
        command_string.pop().unwrap();
        command_string
    } else {
        format!("please login")
    }
}