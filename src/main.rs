use std::collections::HashMap;

fn main() {
    //修改HashMap,直接覆盖
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 1);
    scores.insert(String::from("Blue"), 10);
    println!("{:?}",scores);

    /*
     * 先检查一个key，如果不存在才插入
     * entry()方法,
     * 返回：enum Entry，代表值是否存在
     * 
     * or_insert()方法
     * - 返回：
     * 如果key存在，返回到对应v的一个可变引用
     * 如果key不存在，将方法参数作为k的新值插进去，返回这个值的可变引用
     * 
     */
    scores.entry(String::from("Blue")).or_insert(123);
    scores.entry(String::from("Yellow")).or_insert(123);
    println!("{:?}",scores);


    /*
     *  基于现有的V来更新V
     *  
     */

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map);
}
