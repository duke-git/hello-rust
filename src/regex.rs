use regex::Regex;
fn main() {
    // let re = Regex::new(r"[-_ &]+").unwrap();
    // let re1 = Regex::new(r"[[:alpha:]]+").unwrap();
    // let re1 = Regex::new(r"[[:word:]]").unwrap();
    let re1 = Regex::new(r"[[a-zA-Z][1-9]](.*)").unwrap();
    // let re1 = Regex::new(r"^([^\d]+)\d(.*)").unwrap();

    let s = " foo_& bar- boo^@😄1a1aa ".trim().to_string();

    // let s = re1.replace_all(s.as_str(), " ");
    let cap = re1.captures(s.as_str()).unwrap();
    println!("{:?}", cap);

    // let s = s.to_string();
    // println!("{}", &s);

    // let re2 = Regex::new(r"[^*[:space:]]+").unwrap();

    // let s = re2.replace_all(s.as_str(), "1");

    // println!("{}", s)
}
