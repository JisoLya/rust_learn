use my_project::{Button, Draw, Screen};

fn main() {
    /*
       利用pub关键字可以实现封装
       使用Trait对象来存储不同的类型 -> lib.rs
       Trait对象执行的是动态派发
       - 当Trait约束作用于泛型时，Rust编译器会执行单态化
         - 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应的函数和方法的非泛型实现
         - 通过单态化生成的代码会执行静态派发，在编译时确定运行方法
         
       - 当使用动态派发时，无法在编译过程中确定你调用的是哪一方法，在运行时会执行额外的代码来确定希望调用的方法
       
       Trait对象必须保证对象安全：
       - 方法的返回值类型不是Self
       - 方法中不包含任何泛型类型参数
       //todo 解释一下对象安全
    */

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 100,
                options: vec![String::from("Soya"), String::from("Ros")],
            }),
            Box::new(Button {
                width: 10,
                height: 20,
                label: String::from("Choice"),
            }),
        ],
    };

    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        //绘制
    }
}

