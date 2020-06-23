use rocket_contrib::json::Json;
use diesel::prelude::*;
use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[get("/get")]
pub fn user_read(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::user::dsl::user;
    user.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[get("/get_by_name/<name>")]
pub fn user_read_by_name(conn: DbConn, name: String) -> Result<Json<User>, String> {
    use crate::schema::user::dsl::user;
    let users: Vec<User> = user.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).unwrap();

    if let Some(value) = users.into_iter().find(|s| s.username.eq(&name)) {
        Ok(Json(value))
    } else {
        return Err("No Match User".to_string());
    }
}