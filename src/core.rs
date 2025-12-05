use std::fs;
use std::path::PathBuf;

use mime_guess;

use crate::logger;
use crate::response;
use crate::response::Status;

pub fn normalize_request(request: &str) -> (&str, &str, &str) {
    let lines: Vec<&str> = request.lines().collect();

    if lines.is_empty() {
        return ("", "", "");
    };

    let data: Vec<_> = lines[0].split_whitespace().collect();

    if data.len() < 3 {
        return ("GET", "/400", "HTTP/1.1");
    }

    let method = data[0];
    let path = data[1];
    let http_v = data[2];

    (method, path, http_v)
}

pub fn get_file_content(path: &str, root_dir: &String) -> (Vec<u8>, String, Status) {
    let root_path = PathBuf::from(&root_dir).canonicalize().unwrap();
    let mut intended_path = PathBuf::from(root_dir);

    if path == "/" {
        intended_path.push("index.html");
    } else {
        intended_path.push(path.trim_start_matches("/"));
    }

    match intended_path.canonicalize() {
        Ok(absolute_file_path) => {
            if absolute_file_path.starts_with(&root_path) {
                let mime = mime_guess::from_path(&root_path).first_or_octet_stream();

                match fs::read(&absolute_file_path) {
                    Ok(content) => response::success(content, mime.to_string(), Status::Success),
                    Err(_) => response::error(Status::InternalServerError, Some("File exists but unreadable")),
                }
            } else {
                logger::log(&format!("[SECURITY] Blocked path traversal attempt: {:?}", path));
                response::error(Status::Forbidden, None)
            }
        }
        Err(_) => response::error(Status::NotFound, None),
    }
}
