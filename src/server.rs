use crate::http::{ParseError, Request, Response, StatusCode};
use std::{convert::TryFrom, io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("[ERROR] failed to pare request : {}", e);
        Response::new(StatusCode::BasRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("[INFO] Server listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, stream_addr)) => {
                    println!("[INFO] Client connected from {}", &stream_addr);

                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("[INFO] Received {}", String::from_utf8_lossy(&buffer));

                            let response: Response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>It works</h1>".to_string()),
                                    // )
                                    handler.handle_request(&request)
                                }
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!(
                            "[ERROR] Failed to read from stream {} : {}",
                            &stream_addr, e
                        ),
                    }
                }
                Err(err) => println!("[ERROR] Could not accept connection: {:?}", err),
            }
        }
    }
}
