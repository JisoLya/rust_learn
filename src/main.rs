// use std::fs::File;
// use std::io;
// use std::io::Read;
use std::{fs::File, io, io::Read};
fn main() {
    /*
        传播错误：如何将错误返回给调用者
     */
}

fn read_username_from_file() -> Result<String,io::Error>{
    // let mut f = File::open("hello.txt")?;
    // // Rust提供了?运算符，用来简化传播错误的操作,Result类型后?表示如果Result返回了Ok变体，那么把Ok中的值作为表达式的值输出
    // // 否则，把这个Err(e)返回
    // // let mut f = match f{
    // //     Ok(file) => file,
    // //     Err(e) => return Err(e),
    // // };

    // let mut s = String::new();
    // // match f.read_to_string(&mut s){
    // //     Ok(_) => Ok(s),
    // //     Err(e) => Err(e),
    // // }
    // f.read_to_string(&mut s)?;
    // Ok(s)

    /*
        Trait std::convert::From上的from函数
        - 用于错误之间的转换
        ?与from函数，被?所应用的错误，会隐式的被from函数处理，他所接收的错误类型会被转换为当前函数返回值的错误类型
     */
    //也可以链式调用
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
