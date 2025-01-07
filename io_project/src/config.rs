use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        let query = args.get(1);
        let file_path = args.get(2);

        if query == None {
            return Err("Missing query param".to_string());
        } else if file_path == None {
            return Err("Missing file path".to_string());
        }

        Ok(Config {
            query: query.unwrap().clone(),
            file_path: file_path.unwrap().clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}
