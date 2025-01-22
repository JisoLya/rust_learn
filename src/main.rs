#[derive(Debug)]
enum Sp {
    type1(i32),
    type2(String),
}

fn main() {
    
    let mut v = vec![10,20,123];
    for i in &mut v{
        *i += 20;
    }

    for i in v{
        println!("{}",i);
    }

    //有意思的是，利用枚举可以附带值的特性，我们可以在vector中存储枚举类型来达到存储不同类型的目的
    let example = vec![
        Sp::type1(3),
        Sp::type2(String::from("abcd")),
    ];

    for i in example{
        println!("{:#?}",i);
    }
}
