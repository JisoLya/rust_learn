fn main() {
    let p = Point{data:String::from("test")};
    let y = Point{data:String::from("test1")};
    
    println!("create two test!");
    //虽然不能显示的调用drop方法，但是可以使用std::mem::drop函数来手动的drop
    drop(p);
}

struct Point {
    data:String,
}
//类似于析构函数
impl Drop for Point {
    fn drop(&mut self) {
        println!("drop Point data:{}",self.data);
    }
}