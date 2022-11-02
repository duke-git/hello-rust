fn main() {
    if_condition();
    for_loop();
    while_loop();
    loop_loop();
}

fn if_condition() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn for_loop() {
    //  使用方法	                等价使用方式	                                    所有权
    // for item in collection	    for item in IntoIterator::into_iter(collection)	   转移所有权
    // for item in &collection	    for item in collection.iter()	                   不可变借用
    // for item in &mut collection	for item in collection.iter_mut()	               可变借用
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

fn loop_loop() {
    let mut n = 0;

    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }
}
