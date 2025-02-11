use std::ops::Deref;

fn main() {
    /*
    Deref Trait
    - 实现deref trait使我们可以自定义解引用运算符*的行为
    - 通过实现Deref，智能指针可以通过常规的方法来解引用
     */
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5,*y);
    //报错
    // assert_eq!(5, y);
    assert_eq!(5, *z);
    
    
    //
    let y = MyBox::new(5);
    //为了支持解引用方法，需要实现Deref trait
    assert_eq!(5,*y);
    /*
    函数和方法的隐式解引用转化，假设T实现了Deref Trait
    当某个类型的引用传递给方法或者函数的时候，他的类型与定义的参数类型不匹配，Deref Coercion就会自动发生，
    编译器会经过一系列调用，把他转化为所需的参数类型
     */
    let m = MyBox::new(String::from("Rust"));
    /* 
    &m &MyBox<String>
           |
     deref &String
           |
       deref &str
    */ 
    hello(&m);
    // 下边的a 相当于一系列的deref
    // let a = &(*m)[..];
    hello("Rust");
    
    /*
    解引用与可变性
    当类型和trait在下列三种情况发生时，Rust会执行deref coercion：
    - 当T:Deref<Target=U>，允许 &T转换为&U
    - 当T:DerefMut<Target=U>,允许&mut T转换为&mut U
    - 当T:Deref<Target=U>,允许&mut T 转换为&U
     */
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}