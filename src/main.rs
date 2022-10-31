fn main() {
    // str_convert()
    let v = 9.6_f32 / 3.2_f32;
    println!("{}", v)
}

// str 字符串切片, 硬编码进可执行文件，也无法被修改, UTF-8编码字符串
// String 可增长、可改变且具有所有权的UTF-8编码字符串
fn str_convert() {
    // str is a slice of string
    let a_str: &str = "hello rust";
    // println!("a_str[1..] is {}", a_str[0]);

    // &str -> String
    let a_string = a_str.to_string();
    println!("a_str.to_string(): {}", a_string);

    // String -> &str
    let s1 = String::from("hello rust");
    println!("&s1: {}", &s1);
    println!("&s1[..]: {}", &s1[..]);
    println!("s1.as_str(): {}", s1.as_str());
}

fn string_mutation() {}
