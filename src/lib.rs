use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

//dyn表示动态
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    // println!("Contents :\n {}", content);
    Ok(())
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config{
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_one(){
        let query = "duct";
        let content = "\n
Rust\n
safe, fast, productive.\n
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,content));
    }
}