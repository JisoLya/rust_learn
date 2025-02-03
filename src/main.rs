fn main() {
    /*
      测试函数
      使用cargo test命令会运行所有测试函数
      可以添加自定义的信息，
      assert!(),第一个参数必填，第二个参数可以填自定义的信息
      assert_eq!()和assert_ne! 前两个参数必填，自定义消息作为第三个参数,当然可以使用自定义占位符{}
     */
}

#[test]
fn test_1(){
    println!("hello!");
}

#[test]
fn test_2(){
    assert_eq!(2,3,"不等");
}