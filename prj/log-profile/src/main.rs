use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn print_time() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_millis();
    print!("{now}");
}

fn print_duration(start_time: u128) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_millis();
    print!("{}", now - start_time);
}

fn main() {
    if let Some(start_time) = env::args().nth(1).and_then(|arg| arg.parse().ok()) {
        print_duration(start_time);
    } else {
        print_time();
    }
}
