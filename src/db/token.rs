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

#[get("/get_by_value/<value>")]
pub fn token_read_by_value(conn: DbConn, value: String) -> Result<Json<Token>, String> {
    use crate::schema::token::dsl::token;
    let tokens: Vec<Token> = token.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).unwrap();

    if let Some(value) = tokens.into_iter().find(|t| t.value.eq(&value)) {
        Ok(Json(value))
    } else {
        return Err("No Match Token".to_string());
    }
}

#[get("/get_by_owner/<owner>")]
pub fn token_read_by_owner(conn: DbConn, owner: String) -> Result<Json<Vec<Token>>, String> {
    use crate::schema::token::dsl::token;
    let tokens: Vec<Token> = token.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).unwrap();
    let tokens = tokens.into_iter()
        .filter(|t| t.owner.eq(&owner)).collect::<Vec<Token>>();

    Ok(Json(tokens))
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