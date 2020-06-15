use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::sql_helper::SqlHelper;

pub enum TokenError {
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
                        clients.lock().unwrap().insert(uuid, stream.try_clone().unwrap());
                        thread::spawn(move || receive(stream));
                        println!("device listening: {}", info);
                    } else {
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                }
            }
        }
    }
}

fn receive(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                let response = String::from_utf8(Vec::from(&buf[0..len])).unwrap();
                if response.contains("OK") {
                    println!("operate done");
                } else {
                    println!("operate error");
                }
            },
            Err(_) => {
                println!("device left");
            }
        }
    }
}

fn check_token(stream: &mut TcpStream) -> Result<String, TokenError> {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(len) => {
            let uuid = String::from_utf8(Vec::from(&buf[0..len])).unwrap();
            let cmd = format!(r"SELECT * FROM uuid WHERE value = '{}'"
                              , uuid);

            match SqlHelper::connect().execute_non_query(cmd) {
                Ok(_) => {
                    println!("check token successfully");
                    Ok(uuid)
                }
                Err(_) => {
                    println!("token has no register");
                    return Err(TokenError::NoRegister)
                }
            }
        }
        Err(_) => {
            println!("{} left", stream.peer_addr().unwrap());
            Err(TokenError::NoSendToken)
        }
    }
}