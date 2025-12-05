pub fn error(status: Status, msg: Option<&str>) -> (Vec<u8>, String, Status) {
    let final_message = msg.unwrap_or(status.reason());

    (
        format!("[ERROR] {} - {}", status.as_u16(), final_message).into_bytes(),
        "text/plain".to_string(),
        status
    )
}

pub fn success(content: Vec<u8>, mime_type: String, code: Status) -> (Vec<u8>, String, Status) {
    (content, mime_type, code)
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Success = 200,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
}

impl Status {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn reason(&self) -> &str {
        match self {
            Status::Success => "OK",
            Status::Forbidden => "Forbidden",
            Status::NotFound => "Not Found",
            Status::InternalServerError => "Internal Server Error",
        }
    }
}