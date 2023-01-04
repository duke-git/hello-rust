fn main() {
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
