use rocket::http::{Cookie, Cookies};
use crate::sql_helper::SqlHelper;
use std::borrow::Borrow;

#[get("/get_register_device")]
pub fn get_register_device(cookies: Cookies<'_>) -> String {
    let value: Vec<&str> =
        cookies.iter().map(|a| a.value().split("=").last().unwrap()).collect();

    let username = value.get(1).unwrap();
    let chksum = value.get(0).unwrap();

    "8b71ba1e-d6c2-46bc-9f34-6664bd3d9c19".to_string()
}