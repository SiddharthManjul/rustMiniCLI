use std::env;
use std::process;
use std::fs;

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query: &String = &args[1];
        let file_path: &String = &args[2];

        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In fileS {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Error!");
    println!("{}", contents);
}
