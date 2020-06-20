use rocket_contrib::json::{Json, JsonValue};
use diesel::prelude::*;
use crate::schema::token;
use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name="token"]
pub struct Token {
    pub value: String,
    pub owner: String,
}

#[get("/get")]
pub fn token_read(conn: DbConn) -> Result<Json<Vec<Token>>, String> {
    use crate::schema::token::dsl::token;
    token.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[delete("/delete/<key>")]
pub fn token_delete(conn: DbConn, key: String) -> Result<JsonValue, JsonValue> {
    use crate::schema::token::dsl::token;
    diesel::delete(token.find(key))
        .execute(&conn.0)
        .map_err(|_| {
            json!({"status": "err"})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[post("/create", format = "json", data = "<new_token>")]
pub fn token_create(conn: DbConn, new_token: Json<Token>) -> Result<String, String> {
    use crate::schema::token;
    diesel::insert_into(token::table)
        .values(&new_token.into_inner())
        .execute(&conn.0)
        .map_err(|_err| -> String {
            "Error when inserting".into()
        })
        .map(|_| {
            "Successfully inserted!".into()
        })
}