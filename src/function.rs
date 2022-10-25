fn main() {
    let result = plus_or_minus(1);
    println!("plus_or_minus result is {}", result);
    dead_end();
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
    // return x + 5;
}

/**
 * 发散函数： diverge function
 */
fn dead_end() -> ! {
    panic!("芭比q了！");
}
