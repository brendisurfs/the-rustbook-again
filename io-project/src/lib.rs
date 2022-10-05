use std::env;

#[derive(Debug)]
pub struct Config {
    pub ignore_case: bool,
    pub file_path: String,
    pub query: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
}
