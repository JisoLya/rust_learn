fn main() {
    /*
        Send trait允许线程之间转移所有权
        Rust中几乎所有类型都实现了Send
        但是Rc<T>没有，只适用于单线程
        任何由Send类型组成的类型也被标记为Send
        除了原始指针之外都实现了Send
        
        Sync允许多线程访问，
        如果T是Sync，那么 &T就是Send
    */
}
