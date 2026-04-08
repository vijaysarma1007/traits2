use std::ops::Add;

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let integer_sum = add_two_numbers::<i32>(1, 2);
    println!("{}", integer_sum);

    let float_sum = add_two_numbers::<f32>(1.5, 2.4);
    println!("{}", float_sum);
}
