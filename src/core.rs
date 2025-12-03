use std::fs;
use std::path::PathBuf;

use mime_guess;
use crate::logger;

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

pub fn get_file_content(path: &str, root_dir: &String) -> (Vec<u8>, String, i16, String) {
    let mut full_path = PathBuf::from(&root_dir);

    if path == "/" {
        full_path.push("index.html");
    } else {
        full_path.push(path.trim_start_matches("/"));
    }

    let mime = mime_guess::from_path(&full_path).first_or_octet_stream();

    match fs::read(&full_path) {
        Ok(content) => (
            content,
            mime.to_string(),
            200,
            "OK".to_string(),
        ),
        Err(_) => {
            let err_msg = format!("[ERROR] File not found: {:?}", full_path);
            logger::log(&err_msg);
            (
                err_msg.into_bytes(),
                "text/plain".to_string(),
                404,
                "Not Found".to_string(),
            )
        }
    }
}
