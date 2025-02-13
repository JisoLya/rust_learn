use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    /*
    通过消息来在线程之间传递信息
        channel:包含发送端和接收端
        mpsc::channel()表示multiple producer, single consumer
        返回一个元组，分别是发送端，接收端
     */
   let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        let val = String::from("Hello World");
        tx.send(val).unwrap();
    });
    
    //会一直阻塞线程，直到收到消息
    //try_recv 方法，不会阻塞，当获取到数据会返回Ok，里边包含着数据
    //否则，返回错误
    loop {
        match rx.try_recv() { 
            Ok(val) => {
                println!("{}", val);
                break;
            },
            Err(_) => {
                println!("Waiting message...");
                thread::sleep(Duration::from_secs(1));
            },
        };
    }
}
