use rocket_contrib::json::Json;
use serde::Deserialize;
use rocket::http::{Cookie, Cookies};
use rocket::local::Client;

use crate::sql_helper::SqlHelper;
use crate::DbConn;
use crate::db::user::User;

#[post("/login", format = "json", data = "<info>")]
pub fn login(mut cookies: Cookies<'_>, info: Json<User>) -> String {
    let info = info.into_inner();
    let cmd = format!(r"SELECT * FROM user WHERE username = '{}' AND password = '{}'", info.username, info.password);

    let client = Client::new(super::super::rocket()).unwrap();
    let mut response = client.get("/db/user/get").dispatch();
    println!("{}", response.body_string().unwrap());

    let redirect = match SqlHelper::connect().execute_query(cmd) {
        Ok(res) => {
            if res.count() != 0 {
                let chksum = generate_chksum(info.password.as_bytes());

                let cookie_one = Cookie::build("username", info.username)
                    .expires(time::now())
                    .max_age(time::Duration::minutes(30))
                    .path("/")
                    .finish();

                let cookie_two = Cookie::build("chksum", format!("{}", chksum))
                    .expires(time::now())
                    .max_age(time::Duration::minutes(30))
                    .path("/")
                    .finish();

                cookies.add(cookie_one);
                cookies.add(cookie_two);

                "/device/new_token".to_string()
            } else {
                "/user/login".to_string()
            }
        }
        Err(_) => {
            "/user/login".to_string()
        }
    };
    redirect
}

fn generate_chksum(name: &[u8]) -> u8 {
    let mut chksum = 0;
    for &i in name {
        chksum = (if chksum & 1 == 1 { 0x80 } else { 0 } + (chksum >> 1) + i as u32) & 0xFF;
    }
    chksum as u8
}
