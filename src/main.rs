use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /*
    共享状态的并发
    通过Mutex::new()创建Mutex<T>,Mutex<T>是一个智能指针
    
    访问数据前，通过lock方法来获取锁，会阻塞当前线程，lock可能会失败，返回一个MutexGuard(智能指针，实现了Deref和Drop)
     */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 1..10 {
        //这里会移交所有权,而Rc只适合单线程
        //可以使用Arc<T>来进行原子计数
        // let handle = thread::spawn(move || {
        //    let mut num =  counter.lock().unwrap();
        //     *num += 1;
        // });
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
