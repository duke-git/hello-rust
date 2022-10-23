fn main() {
    // 变量绑定、可变性
    // let mut x = 5;
    // println!("x is {}", x);
    // x = 6;
    // println!("x is {}", x);

    // let _y = 10;

    //变量解构,解构赋值
    // let (a, mut b) = (true, false);
    // println!("a={:?}, b={:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

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
