fn main() {
    statement();
    expression();
}

/**
 * 语句：
 * 1. 完成具体的操作（赋值...)
 * 2. 没有返回值
 */
fn statement() {
    let _a = 1;
    let _vec: Vec<f64> = Vec::new();
    let (_a, _c) = ("hi", false);
}
/**
 * 表达式：
 * 1. 会进行求值
 * 2. 会返回值
 * 3. 不能加分号
 * 4. 如果不返回任何值，会隐式地返回一个 ()
 */
fn expression() {
    let y = {
        let x = 1;
        x + 1 //表达式
              // return x + 1; //mismatched types expected `()`, found integer
    };
    println!("value of y is {}", y)
}
