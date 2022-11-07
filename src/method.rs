fn main() {
    method();
    enum_method();
}

fn method() {
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            return Rectangle { width, height };
        }

        pub fn width(&self) -> u32 {
            return self.width;
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect = Rectangle::new(30, 50);

    println!(
        "width is {}, height is {}, area is {}",
        rect.width(),
        rect.height,
        rect.area()
    );
}

fn enum_method() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    // 为 TrafficLightColor 实现所需的方法
    impl TrafficLightColor {
        fn color(&self) -> String {
            // 在这里定义方法体
            // match *self {
            //     TrafficLightColor::Red => "red",
            //     TrafficLightColor::Yellow => "yellow",
            //     TrafficLightColor::Green => "green",
            // }
            match *self {
                TrafficLightColor::Red => "red".to_string(),
                TrafficLightColor::Yellow => "yellow".to_string(),
                TrafficLightColor::Green => "green".to_string(),
            }
        }
    }
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("c is {:?}", c);
}
