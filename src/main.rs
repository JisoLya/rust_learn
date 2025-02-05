use my_project::Config;
use std::{env, process};

fn main() {
    /*
        项目：
        简单版本的grep
     */
    let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = my_project::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
    //unwrap_or_else 调用一个闭包
}