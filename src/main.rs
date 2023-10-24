use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args())
    .unwrap_or_else(|err| {
        eprintln!("\nProblem parsing arguments: {err}\n");
        process::exit(1);
    });
    
    println!("Searching for \"{}\"", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("\nApplication error: {e}\n");
        process::exit(1);
    }
}