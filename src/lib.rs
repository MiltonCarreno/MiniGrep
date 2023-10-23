use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        return Ok(Config{ query, file_path, ignore_case });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }   
    return matches;
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }   
    return matches;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        // First test
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );

        // Second test
        let query = "st";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Start now.";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive."], 
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        // First test
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

        // Second test
        let query = "St";
        let contents = "\
Rust:
Sting like a bee.
Pick three.
Start now.";
        assert_eq!(
            vec!["Rust:", "Sting like a bee.", "Start now."], 
            search_case_insensitive(query, contents)
        );
    }
}