use rocket::local::Client;
use rocket::http::Cookies;
use std::str::FromStr;
use std::collections::HashMap;
use rocket::http::{ContentType, Status};
use crate::db::token::Token;
use crate::db::user::User;
use crate::user::login::generate_chksum;

#[get("/new_token")]
pub fn new_token(cookies: Cookies<'_>) -> String {
    let uuid = uuid::Uuid::new_v4();
    let mut user = HashMap::new();

    if cookies.iter().count() != 0 {
        for cookie in cookies.iter() {
            user.insert(cookie.name(), cookie.value());
        }

        let username = user.get("username").unwrap();
        let chksum = user.get("chksum").unwrap();

        let uri = format!("/db/user/get/{}", username);
        let client = Client::new(crate::rocket()).unwrap();
        let mut response = client.get(uri).dispatch();

        match serde_json::from_str::<User>(&response.body_string().unwrap()) {
            Ok(user ) => {
                if u8::from_str(chksum).unwrap() == generate_chksum(user.password.as_bytes()) {
                    let new_token = Token {
                        value: uuid.to_string(),
                        owner: username.to_string()
                    };
                    let json = serde_json::to_string(&new_token).unwrap();
                    let res = client.post("/db/token/create").header(ContentType::JSON).body(json).dispatch();
                    assert_eq!(res.status(), Status::Ok);
                    format!("new token {} has been generated, you can add to your device", uuid.to_string())
                } else {
                    format!("please login")
                }
            }
            _ => { format!("please login") }
        }
    } else {
        format!("please login")
    }
}