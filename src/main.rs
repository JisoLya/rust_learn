struct Point<T,V>{
    x: T,
    y: V,
}

impl <T,V> Point<T,V> {
    //处理一个具有U，W泛型的函数
    fn mixup<U,W>(self,other:Point<U,W>) -> Point<U,V>{
        Point{
            x : other.x,
            y : self.y,
        }
    }
}
fn main() {
    /*
        泛型，处理重复代码

     */
    let p1 = Point{x : 5, y : 32};
    let p2 = Point{x: "123",y : 3.0};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}",p3.x,p3.y);
}
