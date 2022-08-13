use crate::http::{ParseError, StatusCode};
use crate::http::{Request, Response};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Error in parsing request: {}", err);
        Response::new(StatusCode::NotFound, None)
    }
}
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => {
                                    handler.handle_bad_request(&err)
                                    // Response::new(StatusCode::NotFound, None)
                                }
                            };
                            if let Err(err) = response.send(&mut stream) {
                                println!("Error in sending response: {}", err);
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                }
                Err(err) => println!("Error in establishing connection: {}", err), // _ =>      acts like default
            }

            // let res = listener.accept();
            // if res.is_err() {
            //     continue;
            // }

            // let (stream, addr) = res.unwrap();
        }
    }
}
