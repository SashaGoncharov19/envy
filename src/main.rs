use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

mod config;
mod core;
mod logger;

const CONFIG_FILE: &str = "config.toml";

fn main() {
    logger::log("Starting envy..");
    config::load(CONFIG_FILE);

    let config = config::get();
    let full_addr = format!("{}:{}", &config.address, &config.port);

    let listener = TcpListener::bind(&full_addr).unwrap();

    logger::log(&format!("Listening at: {}", &full_addr));

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
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 8192];

    let readable_buffer = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return,
    };

    if readable_buffer == 0 { return; }

    let buf_to_str = String::from_utf8_lossy(&buffer[..readable_buffer]);
    let (method, path, http_v) = core::normalize_request(&buf_to_str);

    logger::log(&format!("[INFO] Received: {} {} {}", method, path, http_v));

    let (body, content_type, code, message) = core::get_file_content(path, &config::get().root_dir);

    let headers = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        code,
        message,
        body.len(),
        content_type
    );

    if let Err(e) = stream.write_all(headers.as_bytes()) {
        logger::log(&format!("[ERROR] Failed to send headers: {}", e));
        return;
    }

    if let Err(e) = stream.write_all(&body) {
        logger::log(&format!("[ERROR] Failed to send body: {}", e));
        return;
    }

    stream.flush().unwrap();
}
