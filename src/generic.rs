fn main() {
    struct_generic();
    const_generic();
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}

fn struct_generic() {
    let p1 = Point { x: 1, y: 1.1 };
    let p2 = Point {
        x: "Hello",
        y: "rust",
    };
    let p3 = p1.mixup(p2);
    println!("point p3 is {:?}", p3)
}

fn const_generic() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("array is {:?}", arr)
}
