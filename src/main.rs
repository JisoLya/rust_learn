use std::fs::File;
fn main() {
    // let f = File::open("hello.txt");

    // //如何匹配不同的错误呢?
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Error opening file:{:?}",err)
    //     }
    // };
    
    /*
        unwrap方法,相当于上边的语句

     */
    // let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("错误信息");
}   
