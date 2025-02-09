fn main() {
    /*
    闭包：可以捕获其所在环境的匿名函数
     */
    /*
       闭包捕获环境值的方式
       1. 获取所有权：    FnOnce
       2. 可变借用：      FnMut
       3. 不可变借用      Fn

       所有闭包都实现了FnOnce
       没有移动捕获变量实现了FnMut
       无需可变访问捕获变量的闭包实现了Fn
    */

    /*
       move 关键字，在参数列表前可以强制闭包取得他所使用环境值的所有权
       - 当闭包传递给新线程以移动数据使其归新线程所有时，此技术最有用
    */
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("cant use x here:{:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
