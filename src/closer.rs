fn main() {
    let s = String::from("hello");

    let update_string = || println!("s is {}", s);

    exec1(update_string);
    exec2(update_string);
    exec3(update_string);
}

fn exec1<F: FnOnce()>(f: F) {
    f()
}

fn exec2<F: FnMut()>(mut f: F) {
    f()
}

fn exec3<F: Fn()>(f: F) {
    f()
}
