use std::fs::File;
fn main() {
    /*
        Rust的错误处理机制
        不可恢复的错误与panic!宏
        当执行panic!时：
        - 打印一个错误信息
        - 展开(unwind) 清理调用栈
        - 退出程序    
     */
    // panic!("crash and burn!");

    /*
        可恢复错误与Result枚举
        enum Result<T,E>{
            Ok(T),
            Err(E),
        }
        T:操作成功情况下，Ok变体里返回的数据类型
        E:操作失败的情况下，Err变体里返回的错误类型
     */
    let f = File::open("hello.txt");
    match f{
        Ok(file) => {println!("open success:{:?}",file)},
        Err(data) =>{println!("open fail!err:{}",data)}
    }
    /*
        unwrap 以及 expect
     */
    

}
