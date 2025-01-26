use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening file {:?}",err);
        }
    };
    //如何匹配不同的错误呢?
    
}
