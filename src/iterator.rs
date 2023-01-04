fn main() {
    iterator_demo1();
    iterator_demo2();
}

fn iterator_demo1() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    //values borrowed here after move
    // println!("{:?}", values)

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    let mut values_iter_mut = values.iter_mut();

    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", values);
}

fn iterator_demo2() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v1);
    println!("{:?}", v2);
}
