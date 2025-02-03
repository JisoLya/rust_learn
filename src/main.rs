fn main() {
    /*
    验证错误：should_panic属性
    测试出现panic会测试通过
     */
}

#[test]
//可以指定错误信息，当不是assertion failed 时测试不会通过
#[should_panic(expected = "assertion failed")]
fn test(){
    panic!("111");
    // panic!("assertion failed");
}