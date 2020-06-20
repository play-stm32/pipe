use rocket::http::Cookies;
use std::collections::HashMap;
use rocket::local::Client;
use crate::db::user::User;
use std::str::FromStr;
use crate::user::login::generate_chksum;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use crate::db::token::Token;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct Device {
    token: String,
    online: bool,
}

#[get("/get_register_device")]
pub fn get_register_device(cookies: Cookies<'_>
                           , clients: State<Arc<Mutex<HashMap<String, TcpStream>>>>)
    -> Result<Json<Vec<Device>>, String> {
    let mut user = HashMap::new();
    let clients = clients.lock().unwrap();

    if cookies.iter().count() != 0 {
        for cookie in cookies.iter() {
            user.insert(cookie.name(), cookie.value());
        }

        let username = user.get("username").unwrap();
        let chksum = user.get("chksum").unwrap();

        let uri = format!("/db/user/get_by_name/{}", username);
        let client = Client::new(crate::rocket()).unwrap();
        let mut response = client.get(uri).dispatch();

        match serde_json::from_str::<User>(&response.body_string().unwrap()) {
            Ok(user) => {
                if u8::from_str(chksum).unwrap() == generate_chksum(user.password.as_bytes()) {
                    let uri = format!("/db/token/get_by_owner/{}", username);
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
            _ => { Err(format!("please login")) }
        }
    } else {
        Err(format!("please login"))
    }
}