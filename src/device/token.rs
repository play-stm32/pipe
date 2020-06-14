use actix_web::Responder;
use actix_web::get;

use crate::sql_helper::SqlHelper;

#[get("/new_token")]
pub async fn get_new_token() -> impl Responder {
    let uuid = uuid::Uuid::new_v4();

    let cmd = format!(r"INSERT INTO uuid (value) VALUES ('{}')"
                      , uuid.to_string());

    match SqlHelper::connect().execute_non_query(cmd) {
        Ok(_) => {
            format!("add token: {} to your device", uuid)
        }
        Err(_) => {
            format!("occur error")
        }
    }
}