use std::fs;
use std::error;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query: &String = &args[1];
        let file_path: &String = &args[2];

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {

    let contents = fs::read_to_string(config.file_path)?;
    println!("{}", contents);

    Ok(())
}