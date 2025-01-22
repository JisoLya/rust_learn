fn main() {
    let s1 = String::from("Hello ,");
    let s2 = String::from("world");

    //这里+相当于调用了一个方法
    /*
        fn add(self, s:&str) -> String{...}
        只能把&str添加到String类型上
        这里&s2是String的引用类型，rust使用了解引用强制转换的方法

        而且由于第二个参数是借用类型，第二个参数的所有权会保留，第一个参数没有&符号，函数会获得s1的所有权
        因此打印会报错
     */
    let s3 = s1 + &s2;

    // println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
}
