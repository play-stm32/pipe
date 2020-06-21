use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rocket::local::Client;
use crate::db::token::Token;

pub enum CheckError {
    NoRegister,
    NoSendToken,
}

pub struct Server {
    listen: TcpListener,
}

pub struct ServerBuilder {
    listen: TcpListener,
}

impl ServerBuilder {
    pub fn build(self) -> Server {
        Server {
            listen: self.listen,
        }
    }
}

impl Server {
    pub fn new() -> ServerBuilder {
        let listen = TcpListener::bind("0.0.0.0:1122").expect("could not bind");
        ServerBuilder {
            listen
        }
    }

    pub fn start(&self, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
        println!("listening：{}", self.listen.local_addr().unwrap());

        for stream in self.listen.incoming() {
            match stream {
                Err(e) => eprintln!("{}", e),
                Ok(mut stream) => {
                    let info = stream.peer_addr().unwrap();
                    println!("{} online", info);

                    if let Ok(uuid) = check_token(&mut stream) {
                        let uuid_clone = uuid.clone();
                        clients.lock().unwrap().insert(uuid, stream.try_clone().unwrap());
                        let clients_clone = clients.clone();

                        thread::spawn(move || receive(uuid_clone, stream, clients_clone));
                        println!("device listening: {}", info);
                    } else {
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                }
            }
        }
    }
}

fn receive(uuid: String, mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                let response = String::from_utf8(Vec::from(&buf[0..len])).unwrap();
                if response.contains("OK") {
                    println!("Command executed");
                } else {
                    println!("Command not executed");
                }
            }
            Err(_) => {
                clients.lock().unwrap().remove(&uuid).unwrap();
                println!("device left");
                break;
            }
        }
    }
}

fn check_token(stream: &mut TcpStream) -> Result<String, CheckError> {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(len) => {
            let uuid = String::from_utf8(Vec::from(&buf[0..len])).unwrap();

            let client = Client::new(crate::rocket_inside()).unwrap();
            let uri = format!("/db/token/get_by_value/{}", uuid);
            let mut response = client.get(uri).dispatch();

            match serde_json::from_str::<Token>(&response.body_string().unwrap()) {
                Ok(_) => {
                    println!("check token successfully");
                    Ok(uuid)
                }
                Err(_) => {
                    println!("token has no register");
                    return Err(CheckError::NoRegister);
                }
            }
        }
        Err(_) => {
            println!("{} left", stream.peer_addr().unwrap());
            Err(CheckError::NoSendToken)
        }
    }
}