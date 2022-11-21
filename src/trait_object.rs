fn main() {
    trait_object();
    box_ref_trait_object();
}

trait Draw {
    fn draw(&self) -> String;
}

fn trait_object() {
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) -> String {
            let s = format!(
                "Button: label is {}, width is {}, height is {}",
                self.label, self.width, self.height
            );

            println!("{}", s);
            return s;
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) -> String {
            // 绘制SelectBox的代码
            let s = format!(
                "SelectBox: width is {}, height is {}",
                self.width, self.height
            );

            println!("{}", s);
            return s;
        }
    }

    let components: [Box<dyn Draw>; 2] = [
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];

    for component in components {
        component.draw();
    }
}

// box
fn box_ref_trait_object() {
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    fn draw1(x: Box<dyn Draw>) {
        // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
        x.draw();
    }

    fn draw2(x: &dyn Draw) {
        x.draw();
    }

    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}
