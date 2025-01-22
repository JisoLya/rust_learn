fn main() {
    //字符串不支持索引访问的模式

    let s = String::from("我是11");
    /*
        字节，标量值，字形簇(最接近字母的概念)
     */
    for b in s.bytes(){
        print!("{}+",b)
    }
    println!();
    // 标量值
    for c in s.chars(){
        print!("{}+",c);
    }
    println!()
    //字符串获取字形簇比较麻烦
}
