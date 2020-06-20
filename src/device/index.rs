use rocket::http::Cookies;
use std::collections::HashMap;
use rocket::local::Client;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use rocket_contrib::json::Json;
use crate::db::token::Token;
use crate::cookie::check_status;

#[derive(Serialize, Deserialize)]
pub struct Device {
    token: String,
    online: bool,
}

#[get("/get_register_device")]
pub fn get_register_device(cookies: Cookies<'_>
                           , clients: State<Arc<Mutex<HashMap<String, TcpStream>>>>)
    -> Result<Json<Vec<Device>>, String> {
    let clients = clients.lock().unwrap();

    let status = check_status(&cookies);
    let pass = status.0;
    let username = status.1;

    if pass {
        let uri = format!("/db/token/get_by_owner/{}", username);
        let client = Client::new(crate::rocket_inside()).unwrap();
        let mut response = client.get(uri).dispatch();
        let tokens: Vec<Token> = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        let tokens: Vec<Device> = tokens.iter().map(|t| {
            let token = t.value.clone();
            let online = if let Some(_) = clients.get(&token) { true } else { false };
            Device { token: token.into(), online }
        }).collect();
        Ok(Json(tokens))
    } else {
        Err(format!("please login"))
    }
}