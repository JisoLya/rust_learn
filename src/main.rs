fn main() {
    // 使用format拼接字符串
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");

    let s = format!("{}-{}-{}",s1,s2,s3);
    println!("{}",s);
}
