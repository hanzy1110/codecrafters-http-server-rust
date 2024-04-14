// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
const CRLF: &str = "\r\n";

struct HTTPRequest {
    path: String,
    host: String,
    user_agent: String,
    additional: String
}

impl HTTPRequest {
    fn new(request: String) -> Self {
        let path = request.split(CRLF).nth(0).unwrap().to_owned();

        Self {
            path,
            host:"".to_owned(),
            user_agent: "".to_owned(),
            additional: "".to_owned(),
        }
    }
    fn get_route(&self) -> String {
        self.path.split(" ").nth(1).unwrap().to_owned()
    }
}

fn write_response(response: &str, stream: &mut TcpStream) -> () {
    stream.write(response.as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0;1024];
                println!("accepted new connection");
                let request = match stream.read(&mut buffer) {
                    Ok(_) => {
                        String::from_utf8_lossy(&buffer)
                    },
                    Err(e) => {
                        todo!("Tuve un error")
                    }
                };
                let request = HTTPRequest::new(request.to_string());
                match request.get_route().as_str() {
                    "/" => write_response(OK_RESPONSE, &mut stream),
                    _ => write_response(NOT_FOUND_RESPONSE, &mut stream)
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
