use std::fmt;

fn main() {
    newtype_demo();
    typealias_demo();
}

fn newtype_demo() {
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn typealias_demo() {
    type Meters = u32;

    let x: u32 = 5;
    let y: Meters = 5;

    println!("x + y = {}", x + y);
}
