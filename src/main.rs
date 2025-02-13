use std::sync::mpsc;
use std::thread;

fn main() {
    /*
    通过消息来在线程之间传递信息
        channel:包含发送端和接收端
        mpsc::channel()表示multiple producer, single consumer
        返回一个元组，分别是发送端，接收端
     */
    //可以使用mpsc::Sender::clone()来创建多个发送者
   let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hello World");
        tx.send(val).unwrap();
        //所有权被移交
        // println!("borrowed value {:?}",val);
    });
    
    let received = rx.recv().unwrap();
    println!("{}", received); 
}
