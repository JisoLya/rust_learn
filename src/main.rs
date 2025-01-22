use std::collections::HashMap;

fn main() {
    /*
        HashMap<K,V>
        insert()
        数据都存在heap中
     */
    let mut scores:HashMap<i32, String> = HashMap::new();

    scores.insert(1, "Jisoo".to_string());
    scores.insert(2, "Rose".to_string());

    //使用collect方法
    let teams = vec![String::from("LA"),String::from("CHA")];
    let initial_scores = vec![10,50];

    let team2scores:HashMap<_,_> = 
        teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}",team2scores);
}
