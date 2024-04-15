// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::thread::{spawn, Thread};

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
    additional: Option<String>
}

impl HTTPRequest {
    fn new(request: String) -> Self {
        let path = request.split(CRLF).nth(0).unwrap().to_owned();
        let host = request.clone().split(CRLF).nth(1).unwrap().to_owned();
        let user_agent = request.clone().split(CRLF).nth(2).unwrap().to_owned();
        // let additional = request.clone().split(CRLF).nth(3).unwrap().to_owned();

        Self {
            path,
            host,
            user_agent,
            additional: None
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

fn get_body(route: String) -> String {
    let contents = route.split("echo/");

    let out = contents.collect::<Vec<&str>>().drain(1..).collect();
    println!("RESPONSE BODY {:?}", out);
    out
}

async fn route_request(stream: &mut TcpStream) -> anyhow::Result<()> {
        let mut buffer = [0;1024];
        println!("accepted new connection");
        let request = match stream.read(&mut buffer) {
            Ok(_) => {
                let content = String::from_utf8_lossy(&buffer);
                HTTPRequest::new(content.to_string())
            },
            Err(e) => {
                todo!("Tuve un error")
            }
        };
        let route = request.get_route();
        match route.as_str() {
            "/" => write_response(OK_RESPONSE,  stream),
            "/user-agent" => {
                let user_agent = request.user_agent.split(": ").nth(1).unwrap();
                let content_length = user_agent.len();
                let length_header = format!("Content-Length: {}\r\n", content_length);
                let response = parse_response(OK_RESPONSE, TEXT_PLAIN, &length_header, &user_agent);
                println!("{}", response);
                write_response(&response,  stream);
            },
            v@_ => {
                if v.contains("echo") {
                    let body = get_body(route);
                    let length_header = format!("Content-Length: {}\r\n", body.len());
                    let response = parse_response(OK_RESPONSE, TEXT_PLAIN, &length_header, &body);
                    println!("{}", response);
                    write_response(&response,  stream);
                } else {
                    write_response(NOT_FOUND_RESPONSE,  stream)
                }
            }

        }

    Ok(())
}

#[tokio::main()]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _handled = route_request(&mut stream).await;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
