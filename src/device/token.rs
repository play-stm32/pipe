use rocket::local::Client;
use rocket::http::Cookies;
use rocket::http::{ContentType, Status};
use crate::db::token::Token;
use crate::cookie::check_status;

#[get("/new_token")]
pub fn new_token(cookies: Cookies<'_>) -> String {
    let uuid = uuid::Uuid::new_v4();

    let status = check_status(&cookies);
    let pass = status.0;
    let username = status.1;

    if pass {
        let client = Client::new(crate::rocket()).unwrap();
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