// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";

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
                println!("{}", request);
                let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
