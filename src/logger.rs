use chrono::Utc;

pub fn format_msg(message: &str) -> String {
    let now = Utc::now();
    format!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S%.3f"), message)
}

pub fn log(message: &str) {
    println!("{}", format_msg(message));
}
