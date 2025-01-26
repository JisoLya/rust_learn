use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");

    //如何匹配不同的错误呢?
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}",e),
            },
            other_error => panic!("Error opening file: {:?}",other_error),
        },
    };
    
}
