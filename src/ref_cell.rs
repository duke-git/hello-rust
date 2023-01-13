use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

fn main() {
    cell_demo();
    refcell_demo();
}

fn cell_demo() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;

    x.set(2);
    y.set(3);
    z.set(4);

    println!("x is {}", x.get());

    //
    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);
}

fn refcell_demo() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", on yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
