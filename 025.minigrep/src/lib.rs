
use std::error::Error;
use std::{fs, env};

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}",line);
    }
    Ok(())
}
pub struct Config {
    pub query :String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config {
   pub fn new(args:&[String]) -> Result<Config,&'static str> {
        if args.len()<3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query,filename,case_sensitive})
    }
    
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines()  {  // 将字符串按照行遍历
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
// 大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = vec![];
    for line in contents.lines()  {  // 将字符串按照行遍历
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents =  "\
        Rust:
safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "DucT";
        let contents =  "\
        Rust:
safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search_case_insensitive(query, contents))
    }
}