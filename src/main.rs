fn main() {
    // str_slice();
    // string_mutation();
    str_escape();
}

// 字符串字面量""是切片
fn str_slice() {
    let s = String::from("hello rust");

    let hello = &s[0..5];
    let rust = &s[6..];

    let str = "hello";

    if str == hello {
        println!("str == hello {}", str == hello);
    }

    println!("{}, {}", hello, rust);
}

// 字符串操作
fn string_mutation() {
    let mut s = String::from("Hello ");

    //push
    s.push('r');
    println!("追加字符 push() -> {}", s);

    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);

    //insert
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    //替换(String) replace/replacen/replace_range
    let new_s = s.replace("rust", "Rust");
    println!("替换字符串 replace() -> {}", new_s);

    //删除(String) pop，remove，truncate，clear
    let last_char = s.pop();
    dbg!(last_char);
    dbg!(s);

    //连接 Concatenate
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);
}

fn str_escape() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
