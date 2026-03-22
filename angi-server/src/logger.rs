
use std::time::Instant;
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use colored::*;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn log_startup(
    name: &str,
    version: &str,
    port: i64,
) {
    println!("{}", r#"
 █████╗ ███╗   ██╗ ██████╗ ██╗
██╔══██╗████╗  ██║██╔════╝ ██║
███████║██╔██╗ ██║██║  ███╗██║
██╔══██║██║╚██╗██║██║   ██║██║
██║  ██║██║ ╚████║╚██████╔╝██║
╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝
"#.bright_cyan());

    println!("{}", "========================================".bright_black());
    println!(
        "{} {} {}",
        "🚀 Server:".bold(),
        name.bright_cyan().bold(),
        format!("v{}", version).bright_green()
    );

    println!(
        "{} {}",
        "📡 Port:".bold(),
        format!("{}", port).yellow()
    );

    println!("{}", "========================================".bright_black());
    println!();
}

pub async fn request_logger(
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let start = Instant::now();

    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let response = next.run(req).await;

    let duration = start.elapsed().as_millis();

    println!(
        "{} {} {} {}ms",
        method.to_string().blue().bold(),
        path.white(),
        "→".bright_black(),
        duration.to_string().green()
    );

    response
}

fn now() -> String {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("{}", since_epoch)
}

fn log(level: &str, color: Color, msg: &str) {
    let time = now();

    println!(
        "{} {} {}",
        format!("[{}]", time).bright_black(),
        format!("[{}]", level).color(color).bold(),
        msg
    );
}

#[allow(dead_code)]
pub fn info(msg: impl AsRef<str>) {
    log("INFO", Color::Cyan, msg.as_ref());
}

#[allow(dead_code)]
pub fn warn(msg: impl AsRef<str>) {
    log("WARN", Color::Yellow, msg.as_ref());
}

#[allow(dead_code)]
pub fn error(msg: impl AsRef<str>) {
    log("ERROR", Color::Red, msg.as_ref());
}
