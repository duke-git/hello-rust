fn main() {
    // let s = String::from("hello");

    let mut s = String::from("hello");
    mutable_ref(&mut s);
    nonmutable_ref(&s);

    println!("{}", s)
}

// 不可变引用
// 1. 不可修改
// 2. 同时可以存在多个
// 3. 不能和可变引用同时存在
fn nonmutable_ref(a_string: &String) {
    // a_string.push_str(" rust"); //不可变引用无法修改引用指向的值
    println!("a_string {}", a_string);

    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // 和可变引用同时存在
    println!("{}, {}", r1, r2)
}
// 可变引用
// 1. 可以修改
// 2. 同时只能存在一个
// 3. 不能和不可变引用同时存在
fn mutable_ref(a_string: &mut String) {
    a_string.push_str(" rust"); //不可变引用无法修改引用指向的值

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // let r3 = &s; // 和不可变引用同时存在

    println!("{}", r1)
}

//悬垂引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s;
// } //s离开作用域并被丢弃。其内存被释放
