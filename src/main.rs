use std::thread;
use std::time::Duration;

fn main() {
    /*
    rust 多线程,通过thread::spawn创建一个线程，接受一个闭包作为参数
    
    并且可以使用join.Handle等待所有线程的完成
    
    使用move闭包可以把一个线程的所有权转移到另一个线程当中
     */
    
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("handle thread :{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(|| {
       for i in 1..=5 {
           println!("main thread :{}", i);
           thread::sleep(Duration::from_millis(1));
       } 
    });
    handle.join().unwrap();
}
