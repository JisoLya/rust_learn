fn main() {
    //调用Vec::new()的时候需要指明类型，或者在添加元素的时候编译器会自动推断
    // let v:Vec<i32> = Vec::new();
    //这样会自动推断类型
    let mut v = Vec::new();
    v.push(1);

    // 也可以这样创建，利用宏vec!
    let v2 = vec![1,2,3];

    /*
        当离开作用域时，vector会被自动清理掉
     */
    //访问元素使用索引或者get方法
    // let thrird = &v2[2];
    // println!("The third elment is {}",thrird);

    //利用get来访问超出边界的元素时，会返回None
    match v2.get(1){
        Some(index) => println!("The third elment is {}",index),
        None => println!("There is no third element!"),
    }

    /*
        所有权与借用规则
        不能同时存在一个可变借用和一个不可变的借用
        push方法的形参里边是一个可变借用！
        如果我们要添加一个元素，那么vec在内存中可能被重新分配，那么我们如果继续访问原有的引用，那么会访问到已被清除的内存
     */
    let mut v3 = vec![1,2,3,4,5,6];
    let first = &v3[0];
    // v3.push(1);
    println!("The first element in v3 is {}",first);

}
