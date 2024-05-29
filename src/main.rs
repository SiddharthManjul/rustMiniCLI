use std::env;
use std::process;

use mini_cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In fileS {}", config.file_path);

    if let Err(e) = mini_cli::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
