use std::{rc::Rc, sync::Arc, thread};

fn main() {
    rc_demo1();

    rc_demo2();

    arc_demo();
}

fn rc_demo1() {
    let a = Rc::new(String::from("hello"));
    println!("rc count1 a is {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("rc count2 a is {}", Rc::strong_count(&a));
    println!("rc count1 b is {}", Rc::strong_count(&b));
}

fn rc_demo2() {
    struct Owner {
        name: String,
        // ...其它字段
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
        // ...其它字段
    }

    // 创建一个基于引用计数的 `Owner`.
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });

    // 创建两个不同的工具，它们属于同一个主人
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // 释放掉第一个 `Rc<Owner>`
    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
    // 数据也被清理释放

    println!("Gadget123 {} owned by {}", gadget2.id, gadget2.owner.name);
}

fn arc_demo() {
    let s = Arc::new(String::from("多线程漫游者"));

    for i in 0..10 {
        let s = Arc::clone(&s);

        let handle = thread::spawn(move || println!("{} {}", s, i));
    }
}
