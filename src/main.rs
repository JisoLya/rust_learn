use std::sync::Mutex;

fn main() {
    /*
    共享状态的并发
    通过Mutex::new()创建Mutex<T>,Mutex<T>是一个智能指针
    
    访问数据前，通过lock方法来获取锁，会阻塞当前线程，lock可能会失败，返回一个MutexGuard(智能指针，实现了Deref和Drop)
     */
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
