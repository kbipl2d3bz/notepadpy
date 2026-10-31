use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.get(1).map(String::as_str).unwrap_or("none"));
}
