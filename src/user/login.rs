use rocket_contrib::json::Json;
use crate::sql_helper::SqlHelper;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[post("/user/login", format = "json", data = "<info>")]
pub fn login(info: Json<LoginInfo>) -> String {
    let cmd = format!(r"SELECT * FROM user WHERE username = '{}' AND password = '{}'", info.username, info.password);
    let response = match SqlHelper::connect().execute_query(cmd) {
        Ok(res) => {
            if res.count() != 0 {
                "login successfully".to_string()
            } else {
                "username or password wrong".to_string()
            }
        }
        Err(_) => {
            "db error".to_string()
        }
    };

    response
}