use std::cell::RefCell;

fn main() {
    /*
    RefCell<T> 和Box<T>
    Box<T>:
    在编译阶段强制代码遵守借用规则
    
    RefCell<T>:
    在运行时检查借用规则    
     */
    let v = vec![1,2,3];
    //这里b是不可修改的，通过给vec字段加入Refcell从而达到了可变的目的
    let b = Test{vec: RefCell::new(v),num:1};
    b.vec.borrow_mut().push(4);
    //这段代码在cargo build不会出错，但是cargo run时会出错,因为出现了多个可变借用
    // let a = b.vec.borrow_mut();
    // let c = b.vec.borrow_mut();
    // println!("{:?}, {:?}",a,c);
}

struct Test{
    vec: RefCell<Vec<i32>>,
    num: i32,
}