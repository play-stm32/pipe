use actix_web::{Responder, web, HttpResponse, HttpRequest};
use actix_web::post;
use protocol::protocol::Request;
use std::sync::Mutex;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::Write;

#[post("/command/{uuid}")]
pub async fn send_command(req: HttpRequest,
                          request: web::Json<Request>,
                          clients: web::Data<Mutex<HashMap<String, TcpStream>>>) -> impl Responder {
    let uuid = req.match_info().get("uuid").unwrap_or("none");

    let clients = clients.clone().into_inner();
    let client = clients.lock().unwrap();
    let mut client = client.get(&uuid.to_string()).unwrap();

    client.write(&serde_json::to_vec(&request.into_inner()).unwrap()).unwrap();
    HttpResponse::Ok()
}