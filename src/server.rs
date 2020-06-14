use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;
use uuid::Uuid;
use crate::sql_helper::SqlHelper;

pub struct Server {
    listen: TcpListener,
}

pub struct ServerBuilder {
    listen: TcpListener,
}

impl Server {
    pub fn new() -> ServerBuilder {
        let listen = TcpListener::bind("0.0.0.0:1122").expect("could no bind");
        ServerBuilder {
            listen
        }
    }

    pub fn start(&self) {
        println!("listening：{}", self.listen.local_addr().unwrap());

        for stream in self.listen.incoming() {
            match stream {
                Err(e) => eprintln!("{}", e),
                Ok(stream) => {
                    println!("{} online", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        receive(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            }
        }
    }
}

impl ServerBuilder {
    pub fn build(self) -> Server {
        Server {
            listen: self.listen,
        }
    }
}

fn receive(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                let uuid = String::from_utf8(Vec::from(&buf[0..len])).unwrap();
                let cmd = format!(r"SELECT * FROM uuid WHERE value = '{}'"
                                  , uuid);

                match SqlHelper::connect().execute_non_query(cmd) {
                    Ok(_) => {
                        println!("check device successfully");
                    }
                    Err(_) => {
                        println!("device no register");
                    }
                }
            }
            Err(_) => {
                println!("{} left", stream.peer_addr()?);
                return Ok(());
            }
        }
    }
}