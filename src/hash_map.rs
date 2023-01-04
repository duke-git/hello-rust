use std::collections::HashMap;

fn main() {
    hash_map_move();
    hash_map_operation();
}

fn hash_map_move() {
    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);
    // handsome_boys.insert(&name, age);

    // std::mem::drop(name);
    println!("因为过于无耻，{:?}已经被除名", handsome_boys);
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age);
}

fn hash_map_operation() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // scores.remove(String::from("Blue").as_str());

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("score is {:?}", score);

    // // 覆盖已有的值
    // let old = scores.insert(String::from("Blue"), 20);
    // assert_eq!(old, Some(10));

    // // 查询新插入的值
    // let new = scores.get("Blue");
    // assert_eq!(new, Some(&20));

    // // 查询Yellow对应的值，若不存在则插入新值
    // let v = scores.entry(String::from("Yellow")).or_insert(5);
    // assert_eq!(*v, 5); // 不存在，插入5

    // // 查询Yellow对应的值，若不存在则插入新值
    // let v = scores.entry(String::from("Yellow")).or_insert(50);
    // assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
