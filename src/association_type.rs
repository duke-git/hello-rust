fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

//association type

struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, num_1: &Self::A, num_2: &Self::B) -> bool {
        return (&self.0 == num_1) && (&self.1 == num_2);
    }

    fn first(&self) -> i32 {
        return self.0;
    }

    fn last(&self) -> i32 {
        return self.1;
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    return container.last() - container.first();
}
