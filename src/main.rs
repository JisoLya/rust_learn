use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    /*
    Rc<T>引用计数智能指针
    使用场景：需要在heap上分配数据，这些数据被程序的多个部分读取，但在编译时无法确定那个部分最后使用完这些数据
    Rc::clone(&a) 增加引用计数
    Rc::strong_count(&a)获得引用计数
    
     */
    let a = Cons(5,
        Box::new(Cons(10,
        Box::new(Nil)))
    );
    let b = Cons(3,Box::new(a));
    //这里a的所有权被移交了，可以把枚举中的Box改称Rc
    let c = Cons(2,Box::new(a));
    
    let a = Rc::new(List2::Cons(5,
                        Rc::new(List2::Cons(10,
                            Rc::new(List2::Nil)
                        ))
    ));
    let b = List2::Cons(3,Rc::clone(&a));
    let c = List2::Cons(5,Rc::clone(&a));
}
enum List{
    Cons(i32, Box<List>),
    Nil,
}

enum List2{
    Cons(i32, Rc<List2>),
    Nil,
}