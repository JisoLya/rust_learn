fn main() {
    /*
        泛型，处理重复代码

     */  
}

//需要实现Trait,大概是这样，后面再实现泛型实现相应Trait
fn larget<T>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list{
        if item > largest{
            largest = item;
        }
    };
    largest
}

//结构体中的泛型
struct Point<T>{
    x: T,
    y: T,
}
//方法中的泛型,表示在类型T上实现方法
impl<T> Point<T> {
    fn x1(&self){

    }
}
//只有i32类型的结构体才有这个x2方法，其他泛型没有
impl Point<i32>{
    fn x2(&self){

    }
}


// 枚举中使用泛型
enum ex<T,E> {
    a(T),
    b(E),
}


