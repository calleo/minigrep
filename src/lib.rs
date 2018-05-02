use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let matches = if config.case_sensitive {
        search(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };
    for line_match in matches {
        println!("{}", line_match);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query) ).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|l| l.to_lowercase().contains(&query) ).collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            Ok(Config { query, filename, case_sensitive })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_new() {
        let args: &[String] = &[String::from("minigrep"), String::from("query"), String::from("filename")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.filename, "filename");
        assert_eq!(config.query, "query");
    }

    #[test]
    fn test_config_new_invalid_argument_count() {
        let args: &[String] = &[String::from("minigrep"), String::from("query")];
        let result = Config::new(&args);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err(), Some("not enough arguments"));
    }

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn test_search_case_insensitive() {
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