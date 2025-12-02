use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

mod config;
mod logger;

const CONFIG_FILE: &str = "config.toml";

fn main() {
    logger::log("Starting envy..");

    let config = config::load(CONFIG_FILE);
    let full_addr = format!("{}:{}", config.address, config.port);

    let listener = TcpListener::bind(&full_addr).unwrap();

    logger::log(&format!("Listening at: {}", full_addr));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => logger::log(&format!("[ERROR] Connection failed: {}", e)),
        }
    }

    fn handle_client(mut stream: std::net::TcpStream) {
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).unwrap();

        let body = "Hello from Rust Web Server!";

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            body.len(),
            body
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
