fn main() {
    /*
    智能指针是一种数据结构：
    - 行为和指针类似
    - 拥有额外的元数据和额外的功能
    
    引用计数(reference counting)智能指针类型
    - 通过记录所有者的数量，使一份数据被多个所有者同时拥有，并且在没有使用者时清理掉数据
    智能指针的例子：
    String Vec<T>
    智能指针的实现:
    一般用struct实现，并且实现了Deref 和 Drop两个trait
    
    Deref trait 允许智能指针struct的实例像引用一样使用
    Drop trait 允许自定义智能指针离开作用域时执行的代码
     */
    
    /*
    Box<T>
    常用场景:
        - 在编译时，某类型的大小无法确认，而使用该类型时，上下文却需要知道他的确切大小
        - 当有大量数据，想移交所有权，但需确保在操作时不会被复制
        - 某个值你只关心他是否实现了特定的trait而不关心他的具体类型
        
     */
    let b = Box::new(5);
    println!("{}", b);
}
enum List{
    Cons(i32, Box<List>),
    Nil,
}