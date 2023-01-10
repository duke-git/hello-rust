use std::ops::Deref;

fn main() {
    let y = MyBox::new(5);
    assert_eq!(5, *y);

    let s = MyBox::new(String::from("hello world"));
    display(&s)
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn display(s: &str) {
    println!("{}", s)
}
