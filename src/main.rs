fn main() {
    // variable_declare();

    // variable_destruct();

    // variable_shadow();

    // variable_int_float();

    variable_char_bool_tuble();
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
        let x = x * 2;
        println!("innter x is: {}", x)
    }

    println!("outter x is: {}", x)
}

/**
 * rust基本数据类型
 * 数值类型: 有符号整数 (i8, i16, i32, i64, isize)、
 * 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
 * 字符串：字符串字面量和字符串切片 &str
 * 布尔类型： true和false
 * 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
 * 单元类型: 即 () ，其唯一的值也是 ()
 */
fn variable_int_float() {
    // // 编译器会进行自动推导，给予twenty i32的类型
    // let twenty = 20;

    // 通过类型后缀的方式进行类型标注：10是i32类型
    let int_num = 10i32; //默认i32
                         // 类型标注
    let float_num: f64 = -1.23; //默认f64
    println!("intNum is {}, floatNum is {}", int_num, float_num);

    // NaN类型：数学上未定义的结果
    // let nan = float_num.sqrt(); // NaN
    // assert_eq!(nan, nan);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    //运算: +, -, *, /, %

    // 位运算：&位与, |位或, ^异或, !位非, <<左移, >>右移

    // 序列Range 1..5, 1..=5
    for i in 1..=5 {
        println!("{}", i);
    }
}


fn variable_char_bool_tuble() {
    //char
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("c is {}, z is {}, g is {}, heart_eyed_cat is {}", c, z, g, heart_eyed_cat);

    //bool
    let t = true;
    let f:bool = false;

    if f || t {
        println!("hello rust");
    }

    //unit
    let unit = ();
    println!("unit is {:?}", unit);

}