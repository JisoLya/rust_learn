use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    /*
        循环引用导致内存泄露
     */
    let a = Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
    
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    
    let b = Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    /*
    这里出现了 a -> b -> a ....无限循环,stack overflow
     */
    // println!("{:?}",a.tail());
}

#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self { 
            Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}