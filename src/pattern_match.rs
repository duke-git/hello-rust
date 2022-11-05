fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    match_demo1(coin);

    match_let();

    match_deconstruction();

    match_bind();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn match_demo1(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_let() {
    let val = Some(1);

    if let Some(1) = val {
        println!("three");
    }

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    // variable desturct/overide
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);
}

//模式解构
fn match_deconstruction() {
    //解构struct
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    //解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn match_bind() {
    // enum Message {
    //     Hello { id: i32 },
    // }

    // let msg = Message::Hello { id: 5 };

    // match msg {
    //     Message::Hello {
    //         id: id_variable @ 3..=7,
    //     } => {
    //         println!("Found an id in range: {}", id_variable)
    //     }
    //     Message::Hello { id: 10..=12 } => {
    //         println!("Found an id in another range")
    //     }
    //     Message::Hello { id } => {
    //         println!("Found some other id: {}", id)
    //     }
    // }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
