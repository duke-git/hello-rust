fn main() {
    box_demo1();

    box_demo2();

    box_leak_demo();
}

fn box_demo1() {
    let arr = [0; 1000];

    let arr1 = arr;

    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0; 1000]);
    let arr1 = arr;

    // println!("{:?}", arr.len());
    println!("{:?}", arr1.len());
}

fn box_demo2() {
    let arr = vec![Box::new(1), Box::new(2)];

    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;

    println!("sum is {}", sum)
}

fn box_leak_demo() {
    let s = gen_static_str();
    println!("{}", s);
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello world");

    Box::leak(s.into_boxed_str())
}
