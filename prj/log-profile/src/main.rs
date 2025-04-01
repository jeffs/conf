use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_millis();
    match env::args().nth(1).and_then(|arg| arg.parse::<u128>().ok()) {
        Some(start_time) => println!("{:4}ms", now - start_time),
        None => println!("{now}"),
    }
}
