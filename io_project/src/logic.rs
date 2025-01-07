use std::fs;

use crate::config::{self, Config};

pub fn run(args: Vec<String>) {
    let Config {
        query,
        file_path,
        ignore_case,
    } = Config::new(args).unwrap();

    let content = fs::read_to_string(file_path).expect("Invalid file path");

    let lines = if ignore_case {
        search_case_insensitive(&query, &content)
    } else {
        search(&query, &content)
    };
    for line in lines {
        println!("{}", line);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_results() {
        let query = "fast";
        let content = "\
            Rust:
safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn insensitive_search() {
        let query = "duct";
        let content = "\
            Rust:
safe, FAST, productive.
            Pick three.";

        assert_eq!(vec!["safe, FAST, productive."], search(query, content));
    }
}
