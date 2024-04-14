// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n";
const ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n";
const TEXT_PLAIN: &str = "Content-Type: text/plain\r\n";
const CRLF: &str = "\r\n";

#[derive(Debug)]
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
    let response = format!("{}\r\n", response);
    stream.write(response.as_bytes()).unwrap();
}
fn parse_response(status: &str, content_type: &str, length_header: &str, body: &str) -> String {
    format!("{status}{content_type}{length_header}\r\n{body}")
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
                println!("{:?}", request);
                let route = request.get_route();
                println!("route {:?}", route.split("/").collect::<Vec<&str>>());
                match route.as_str() {
                    "/" => write_response(OK_RESPONSE, &mut stream),
                    v@_ => {
                        if v.contains("echo") {
                            println!("route contains echo");
                            let body = route.split("/").last().unwrap();
                            let length_header = format!("Content-Length: {}\r\n", body.len());
                            let response = parse_response(OK_RESPONSE, TEXT_PLAIN, &length_header, &body);
                            println!("{}", response);
                            write_response(&response, &mut stream);
                        } else {
                            write_response(NOT_FOUND_RESPONSE, &mut stream)
                        }
                    }

                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
