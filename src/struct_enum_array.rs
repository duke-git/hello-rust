fn main() {
    // struct_demo();
    // enum_demo();
    // enum_option(Some(5));
    array_demo();
}

fn struct_demo() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1 is {:?}", user1)
}

fn enum_demo() {
    #[derive(Debug)]
    enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(u8),
        Hearts(u8),
    }

    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);

    println!("c1 is {:?}, c2 is {:?}", c1, c2);

    let card_num = match c1 {
        PokerCard::Spades(value) => Some(value),
        _ => None,
    }
    .unwrap();

    println!("number is {}", card_num);
}

fn enum_option(x: Option<i32>) {
    match x {
        None => {
            println!("No value")
        }
        Some(i) => {
            println!("value is {}", i)
        }
    }
}

fn array_demo() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
