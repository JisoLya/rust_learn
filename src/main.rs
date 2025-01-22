fn main() {
    //字符串的切割
    let s = "我是Jisoo";
    //4不是字符的边界,这里切割会panic
    // let split = &s[..4];
    // println!("{}",split);
    let sp = &s[..6];
    println!("{}",sp);

    //遍历String, 字节bytes() 标量值chars()
}
