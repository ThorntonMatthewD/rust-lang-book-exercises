use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let result = match args.len() {
            1..=2 => Err("Not enough arguments provided!"),
            4.. => Err("Too many arguments provided!"),
            3 => {
                Ok(Config {
                    query: args[1].clone(),
                    file_path: args[2].clone()
                })
            }
            _ => unreachable!()
        };
        
        result
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in file {}", config.query, config.file_path);
    
    let contents = fs::read_to_string(config.file_path)?; 

    println!("File contents:\n{contents}");

    Ok(())
}       
