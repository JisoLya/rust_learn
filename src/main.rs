fn main() {
    /*
    模式匹配
    if let
    else if let
    else...

    while let ..
     */
    let mut vec1 = vec![1, 2, 3];
    while let Some(numer) = vec1.pop() {
        println!("{}", numer);
    }

    /*
    匹配字面值
     */
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    /*
    匹配命名变量
    命名的变量是可匹配任何值的无可辩驳模式
     */
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched {:?}", y),
        _ => println!("Default case: x = {:?}, y = {:?}", x, y),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    /*
    使用 1|2匹配 1或者2
     */
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("other"),
    }

    /*
    ..=匹配某个范围的值
    */
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("other"),
    }

    /*
    结构以分解值，可以使用模式来解构struct，enum，tuple
    来使用不同部分
     */
    let p = Point { x: 23, y: 24 };

    // let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    /*
    可以用 _ 忽略值或者忽略值的某一部分
     */
    let s = Some(String::from("hello"));
    //这样所有权也不会发生转移
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    /*
    使用match守卫来提供额外的条件
     */
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    /*
    @ 符号让我们创建一个变量，该变量可以在测试某个值是否匹配模式的同时保存该值
     */
    let msg = Message::Hello { id: 5 };
    let msy = Message::Tome(5);
    match msg{
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
        Message::Tome(y) => println!("y in Tome is {}",y),
    }

    match msy{
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
        Message::Tome(y) => println!("y in Tome is {}",y),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
    Tome(i32),
}
