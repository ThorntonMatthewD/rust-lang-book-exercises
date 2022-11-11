use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let result = match args.len() {
            1..=2 => Err("Not enough arguments provided!"),
            4.. => Err("Too many arguments provided!"),
            3 => {
                Ok(Config {
                    query: args[1].clone(),
                    file_path: args[2].clone(),
                    ignore_case: env::var("IGNORE_CASE").is_ok()
                })
            }
            _ => unreachable!()
        };

        result
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_results: Vec<&str> = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents)
    };

    for result in search_results{
        println!("{result}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let matches: Vec<&'a str> = contents.lines()
        .filter(|line| line.contains(query))
        .collect();

    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let matches: Vec<&'a str> = contents.lines()
        .filter(|line|
            line.to_lowercase()
                .contains(&query.to_lowercase())
        )
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

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
