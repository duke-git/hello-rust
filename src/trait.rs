use std::fmt::Debug;
fn main() {
    impl_trait();
    trait_param();
    trait_bound();
}

pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Read more...");
    }
}
#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        return format!("文章{}, 作者是{}", self.title, self.author);
    }
}

#[derive(Debug)]
pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博: {}", self.username, self.content)
    }
}

fn impl_trait() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "test1".to_string(),
        content: "Hello rust!".to_string(),
    };
    let weibo = Weibo {
        username: "test2".to_string(),
        content: "微博内容".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}

fn trait_param() {
    let weibo = Weibo {
        username: "test2".to_string(),
        content: "微博内容".to_string(),
    };

    notify_1(&weibo);
}

fn notify_1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn trait_bound() {
    let post1 = Post {
        title: "Rust语言简介".to_string(),
        author: "test1".to_string(),
        content: "Hello rust!".to_string(),
    };
    let post2 = Post {
        title: "C语言简介".to_string(),
        author: "test2".to_string(),
        content: "Hello C!".to_string(),
    };

    let weibo = Weibo {
        username: "test3".to_string(),
        content: "微博内容".to_string(),
    };

    // notify_2(&post1, &post2);
    // notify_3(&post1, &post2);
    notify_4(&post1, &weibo);
}

//特征约束
fn notify_2<T: Summary>(item1: &T, item2: &T) {
    println!("item1 summary! {}", item1.summarize());
    println!("item2 summary! {}", item2.summarize());
}

//多重约束
fn notify_3<T: Summary + Debug>(item1: &T, item2: &T) {
    println!("item1 is! {:?}", item1);
    println!("item1 summary! {}", item1.summarize());

    println!("item2 is! {:?}", item2);
    println!("item2 summary! {}", item2.summarize());
}

// where 约束
fn notify_4<T, U>(item1: &T, item2: &U)
where
    T: Summary + Debug,
    U: Summary + Debug,
{
    println!("item1 is! {:?}", item1);
    println!("item1 summary! {}", item1.summarize());

    println!("item2 is! {:?}", item2);
    println!("item2 summary! {}", item2.summarize());
}

//条件约束

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }
