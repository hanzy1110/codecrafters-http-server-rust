use std::env;
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::path::PathBuf;
use std::thread::{self, spawn, Thread};

use regex::Regex;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n";
const ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n";
const TEXT_PLAIN: &str = "Content-Type: text/plain\r\n";
const OCTECT_STREAM: &str = "Content-Type: application/octet-stream\r\n";
const CRLF: &str = "\r\n";

const FILE_RQ_PATTERN: &str = r"/files/(?<path>.*?)";

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

fn route_request(stream: &mut TcpStream, path: PathBuf) -> anyhow::Result<()> {
        let files_re = Regex::new(FILE_RQ_PATTERN).unwrap();
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
                } else if let Some(caps) = files_re.captures(v) {
                    let filepath = &caps["path"];
                    let complete_path = path.join(filepath);
                    println!("COMPLETE PATH =>> {:?} - FILE PATH {:?} -- ROUTE {:?}", complete_path, filepath, route.as_str());
                    if complete_path.exists() {
                        let body = std::fs::read_to_string(complete_path).unwrap();
                        let length_header = format!("Content-Length: {}\r\n", body.len());
                        let response = parse_response(OK_RESPONSE, OCTECT_STREAM, &length_header, &body);
                        println!("{}", response);
                        write_response(&response,  stream);
                    } else {
                        write_response(NOT_FOUND_RESPONSE,  stream)
                    }
                } else {
                    write_response(NOT_FOUND_RESPONSE,  stream)
                }
            }

        }

    Ok(())
}

// #[tokio::main()]
fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // Uncomment this block to pass the first stage
    //
    let mut path = PathBuf::new();
    let args = env::args().collect::<Vec<String>>();
    let got_dir = args.get(1).map(|v|{
        if v.starts_with("--directory") {
            Some(v)
        } else {
            None
        }
    }).is_some();

    if got_dir {
        path = path.join(args.get(2).unwrap());
        println!("PATH PROVIDED ==> {:?}", path);

    } else {
        println!("ERROR NO PATH PROVIDED!!");
        std::process::exit(1);
    }


    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let mut threads = vec![];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {

                println!("Accepted Connection...");
                let path = path.clone();
                threads.push(thread::spawn(move || route_request(&mut stream, path)));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
