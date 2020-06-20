use rocket::http::Cookies;

#[get("/get_register_device")]
pub fn get_register_device(cookies: Cookies<'_>) -> String {
    "8b71ba1e-d6c2-46bc-9f34-6664bd3d9c19".to_string()
}