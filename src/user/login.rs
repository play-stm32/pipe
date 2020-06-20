use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use rocket::local::Client;
use crate::db::user::User;

#[post("/login", format = "json", data = "<info>")]
pub fn login(mut cookies: Cookies<'_>, info: Json<User>) -> String {
    let info = info.into_inner();

    let uri = format!("/db/user/get_by_name/{}", info.username);
    let client = Client::new(crate::rocket()).unwrap();
    let mut response = client.get(uri).dispatch();

    let redirect = match serde_json::from_str::<User>(&response.body_string().unwrap()) {
        Ok(user) => {
            if user.password.eq(&info.password) {
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

pub fn generate_chksum(name: &[u8]) -> u8 {
    let mut chksum = 0;
    for &i in name {
        chksum = (if chksum & 1 == 1 { 0x80 } else { 0 } + (chksum >> 1) + i as u32) & 0xFF;
    }
    chksum as u8
}
