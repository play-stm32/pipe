use std::collections::HashMap;
use rocket::http::Cookies;
use rocket::local::Client;
use std::str::FromStr;
use crate::db::user::User;

pub fn check_status(cookies: &Cookies<'_>) -> (bool, String) {
    let mut user = HashMap::new();
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
                (u8::from_str(chksum).unwrap() == generate_chksum(user.password.as_bytes()), username.to_string())
            }
            _ => { (false, "".to_string()) }
        }
    } else {
        (false, "".to_string())
    }
}

pub fn generate_chksum(name: &[u8]) -> u8 {
    let mut chksum = 0;
    for &i in name {
        chksum = (if chksum & 1 == 1 { 0x80 } else { 0 } + (chksum >> 1) + i as u32) & 0xFF;
    }
    chksum as u8
}
