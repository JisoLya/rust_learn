fn main() {
    let s = "Hello world".to_string();
    let mut s2 = String::from("123");
    //字符串切片&str有点类似与java中的字符串常量池中的一个借用，在内存当中
    
    //push_str不会获取参数字符串切片的所有权
    s2.push_str("122");
    println!("{}",s2);

    //push():把一个字符附加到String后面
    
}
