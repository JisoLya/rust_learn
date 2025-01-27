use std::fmt::{format, Display};

fn main() {
    //Trait
    /*
        Trait告诉Rust编译器，某种类型具有那些并且可以与其他类型共享的功能
        Trait定义了抽象的共享行为
        Trait bounds: 泛型类型参数指定为实现了特定行为的类型
     */
    let p1 = Point{x: 1,y :2};
    println!("{}",p1.summarize());
}

//定义一个trait(类似于其他语言的接口)
pub trait Summary {
    //给出默认实现,并且我们可以在默认实现里边调用没有实现的方法,而实现了这个trait的类型必须实现这个方法！(模板方法模式)
    fn summarize(&self) -> String{
        format!("default + {}", self.no_impl())
    }
    fn no_impl(&self) -> String;
}

// 在类型上实现Trait
/*
    实现Trait的约束
    - 这个类型或者个trait是在本地crate里定义的
*/
struct Point{
    x:i32,
    y:i32,
}
impl Summary for Point {
    fn summarize(&self) -> String {
        format!("x = {}, y = {}",self.x,self.y)
    }
    fn no_impl(&self) -> String {
        format!("123")
    }
}
//表示实现了某个trait的类型
pub fn notify(item: impl Summary) -> impl Summary{
    item
}
//实现了多个Trait,返回类型必须实现了Summay类型
pub fn notify1(item: impl Summary + Display) -> impl Summary{
    item
}

//Trait bound语法
pub fn notify2<T: Summary>(item: T, ite2: T) -> (T,T){
    (item,ite2)
}
//用+的方式
pub fn notify3<T: Summary + Display>(item: T, ite2: T) -> (T,T){
    (item,ite2)
}

//也可以用where字句，简化Trait bound
pub fn notify4<T,U>(item: T,ite2: U) -> String
    where T: Summary + Display,U: Display,
{
    format!("asd")
}