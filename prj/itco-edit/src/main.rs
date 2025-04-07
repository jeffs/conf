use std::env;

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "hx".to_owned());
        println!("{editor} '{arg}'");
    }
}
