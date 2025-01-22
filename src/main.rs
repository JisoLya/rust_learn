use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //对于实现了copy trait的类型，值会被复制到HashMap中，而所有权不会改变，
    //对于拥有所有权的值，值会被移动，所有权会转移到HashMap中
    //如果把值的引用存入到HashMap中，那么所有权不会改变

    // println!("{} : {}",field_name,field_value);


    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    scores.insert(&blue, 10);
    scores.insert(&yellow, 50);

    /*
        访问HashMap中的值
        get(key)方法
        返回Option<&V>
     */
    match scores.get(&blue){
        Some(s) => println!("{}",s),
        None => println!("Team does not exist")
    }

    /*
        遍历HashMap
     */
    for (k, v) in scores{
        println!("key = {}, value = {}",k,v);
    }
}
