use std::error::Error;
use std::{env, fs};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

//dyn表示动态
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for val in result {
        println!("{}", val);
    }
    // println!("Contents :\n {}", content);
    Ok(())
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        //如果传入了命令，那么从命令中获取
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = match args.next() {
            Some(arg) => {
                if arg == "--case-insensitive"{
                    false
                }else{
                    true
                }
            },
            None => {
                env::var("CASE_INSENSITIVE").is_err()
            }
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
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
    fn find_one() {
        let query = "ust";
        let content = "\n
Rust\n
safe, fast, productive.\n
Pick three.\n\
Usta";
        assert_eq!(
            vec!["Rust", "Usta"],
            search_case_insensitive(query, content)
        );
    }
}
