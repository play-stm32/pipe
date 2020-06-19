use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel::RunQueryDsl;
use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[get("/user/get")]
pub fn user_read(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::user::dsl::user;
    user.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}