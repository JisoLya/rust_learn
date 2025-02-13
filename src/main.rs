use std::thread;

fn main() {
    /*
    rust 多线程,通过thread::spawn创建一个线程，接受一个闭包作为参数

    并且可以使用join.Handle等待所有线程的完成

    使用move闭包可以把一个线程的所有权转移到另一个线程当中
     */
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    let handle = thread::spawn(move ||{
        println!("{:?}", v);
    });
    
    handle.join().unwrap();
}
