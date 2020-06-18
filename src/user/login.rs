use rocket_contrib::json::Json;
use serde::Deserialize;
use rocket::http::{Cookie, Cookies};
use crate::sql_helper::SqlHelper;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[post("/user/device", format = "json", data = "<info>")]
pub fn login(mut cookies: Cookies<'_>, info: Json<LoginInfo>) -> String {
    let info = info.into_inner();
    let cmd = format!(r"SELECT * FROM user WHERE username = '{}' AND password = '{}'", info.username, info.password);

    let redirect = match SqlHelper::connect().execute_query(cmd) {
        Ok(res) => {
            if res.count() != 0 {
                cookies.add(Cookie::new("username", info.username));
                "/device/new_token".to_string()
            } else {
                "/user/device".to_string()
            }
        }
        Err(_) => {
            "/user/device".to_string()
        }
    };
    redirect
}