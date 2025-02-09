fn main() {
    /*
        迭代器模式：对一系列项执行某些任务
        
        Rust的迭代器:
        - 懒惰的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果
     */
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    /*
    iterator trait
    所有迭代器都实现了iterator trait
    大致定义如下
    pub trait Iterator{
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
        // 其他方法...
    }
    - type Item 与 Self::Item 定义了与此trait关联的类型
    
    几个迭代方法
    - iter方法：在不可变引用上创建迭代器
    - into_iter方法: 创建迭代器会获得所有权
    - iter_mut方法： 迭代器的可变引用
     */
    
    /*
    消耗迭代器的方法：
    调用next()的方法被称为消耗型适配器，调用他们会把迭代器消耗尽
    e.g.
        sum()方法：
        - 取得迭代器所有权
        - 反复调用next()
        - 每次迭代把当前元素添加到一个总和中，迭代结束，返回总和
     */
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("{}", total);
    
    /*
    产生其他迭代器的方法
    把迭代器转换为不同种类的迭代器成为迭代器适配器
    例如map：
    接受一个闭包，闭包作用于每个元素
    产生一个新的迭代器
    
    collect方法，消耗型适配器，把结果收集到一个集合类型中
     */
    
    let v2: Vec<i32> = vec![1, 2, 3];
    let v2_iter:Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!("{:?}", v2_iter);
}
