use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case =
            match args.get(3) {
                Some(value) if value == "--case_sensitive" => false,
                Some(value) if value == "--case_insensitive" => true,
                Some(_) => return Err(
                    "Invalid flag. Expected \"--case_insensitive\" or \"--case_sensitive\" flag",
                ),
                None => env::var("IGNORE_CASE").is_ok(),
            };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let search_results: Vec<&str> = content
        .lines()
        .into_iter()
        .filter(|&line| line.contains(query))
        .collect();

    search_results
}

fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let search_results: Vec<&str> = content
        .lines()
        .into_iter()
        .filter(|&line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    search_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "something unusual";
        let content = "\
RUST:
something unusual
Fix it please or go home";

        assert_eq!(
            vec!["something unusual"],
            search_case_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
RUST:
something unusual
Fix it please or go home
Trust me";
        assert_eq!(
            vec!["RUST:", "Trust me"],
            search_case_insensitive(query, content)
        );
    }
}
