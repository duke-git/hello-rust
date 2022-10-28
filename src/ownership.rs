fn main() {
    ownership_demo1();
    ownership_demo2();
}

fn ownership_demo1() {
    let s = String::from("hello");
    takes_heap_variable_ownership(s);
    // println!("{}", s) //value borrowed here after move: 无法访问s. 编译错误，s被移动到takes_ownership函数里面

    let x = 5;
    make_stack_variable_copy(x);
    println!("{}", x); //i32是Copy的, 所以在后面可继续使用 x
}

//
fn takes_heap_variable_ownership(a_string: String) {
    println!("{}", a_string)
}
// 浅拷贝只生在栈上
// 所有整数类型，比如 u32。
// 布尔类型，bool，它的值是 true 和 false。
// 所有浮点数类型，比如 f64。
// 字符类型，char。
// 元组，当且仅当其包含的类型也都是Copy的时候。比如，(i32, i32) 是Copy的，但(i32, String)就不是。
// 不可变引用 &T, 可变引用 &mut T 是不可以 Copy的
fn make_stack_variable_copy(a_integer: i32) {
    println!("{}", a_integer)
}

fn ownership_demo2() {
    let s1 = give_ownership();
    println!("{}", s1);

    let s2 = String::from("hello2");
    let s3 = takes_and_gives_back(s2);

    // println!("{}", s2); //s2不可访问，被移动到了takes_and_gives_back函数里面
    println!("{}", s3);
}
fn give_ownership() -> String {
    let a_string = String::from("hello1");
    a_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
