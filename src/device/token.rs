use crate::sql_helper::SqlHelper;

#[get("/new_token")]
pub fn new_token() -> String {
    let uuid = uuid::Uuid::new_v4();

    let cmd = format!(r"INSERT INTO uuid (value) VALUES ('{}')"
                      , uuid.to_string());

    match SqlHelper::connect().execute_query(cmd) {
        Ok(_) => {
            format!("add token: {} to your device", uuid)
        }
        Err(_) => {
            format!("occur error")
        }
    }
}