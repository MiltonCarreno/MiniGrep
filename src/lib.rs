use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        return Ok(Config{ query, file_path });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    // println!("\nWith text: \n{contents}");

    for line in search(&config.query, &contents) {
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

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );
    }

    #[test]
    fn two_results() {
        let query = "st";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive."], 
            search(query, contents)
        );
    }
}