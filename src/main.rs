fn main() {
    variable_declare();
    variable_destruct();

    variable_shadow();
}

// 变量绑定、可变性
fn variable_declare() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    let _y = 10;
}

// 变量解构,解构赋值
fn variable_destruct() {
    let (a, mut b) = (true, false);
    println!("a={:?}, b={:?}", a, b);
    b = true;
    assert_eq!(a, b);

    struct Struct {
        e: i32,
    }
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

/**
 * 变量遮蔽:
 * let生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
 * 而mut声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好
 */
fn variable_shadow() {
    let x = 5;

    let x = x + 1;
    {
        let x = x*2;
        println!("innter x is: {}", x)
    }

    println!("outter x is: {}", x)

}
