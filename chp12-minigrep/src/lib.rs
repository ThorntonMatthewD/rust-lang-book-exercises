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
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let matches: Vec<&'a str> = contents.lines()
        .filter(|line| line.contains(query))
        .collect();

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
