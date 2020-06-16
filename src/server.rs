use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::sql_helper::SqlHelper;

pub enum CheckError {
    NoRegister,
    NoSendToken,
    QueryError
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
            },
            Err(_) => {
                clients.lock().unwrap().remove(&uuid).unwrap();
                println!("device left");
            }
        }
    }
}

fn check_token(stream: &mut TcpStream) -> Result<String, CheckError> {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(len) => {
            let uuid = String::from_utf8(Vec::from(&buf[0..len])).unwrap();
            let cmd = format!(r"SELECT * FROM uuid WHERE value = '{}'"
                              , uuid);

            match SqlHelper::connect().execute_query(cmd) {
                Ok(res) => {
                    if res.count() != 0 {
                        println!("check token successfully");
                        Ok(uuid)
                    } else {
                        println!("token has no register");
                        return Err(CheckError::NoRegister)
                    }
                }
                Err(_) => {
                    return Err(CheckError::QueryError)
                }
            }
        }
        Err(_) => {
            println!("{} left", stream.peer_addr().unwrap());
            Err(CheckError::NoSendToken)
        }
    }
}