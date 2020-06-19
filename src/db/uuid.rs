use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel::RunQueryDsl;
use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Uuid {
    pub uuid: String,
    pub owner: String,
}

#[get("/uuid/get")]
pub fn uuid_read(conn: DbConn) -> Result<Json<Vec<Uuid>>, String> {
    use crate::schema::uuid::dsl::uuid;
    uuid.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}