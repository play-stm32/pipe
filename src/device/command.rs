use protocol::protocol::Request;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::Write;
use rocket::State;
use rocket_contrib::json::Json;

#[post("/device/command/<uuid>", format = "json", data = "<request>")]
pub fn send_command(uuid: String, request: Json<Request>
                    , clients: State<Arc<Mutex<HashMap<String, TcpStream>>>>) -> String {
    let client = clients.lock().unwrap();

    if let Some(mut client) = client.get(&uuid) {
        client.write(&serde_json::to_vec(&request.into_inner()).unwrap()).unwrap();
        "Ok".to_string()
    } else {
        "No Match Device".to_string()
    }
}