fn print_slice(slice: &[i32]) {
    println!("{:?}", slice);
}

fn main() {
    let string_slice: &[String] = &["One".to_owned(), "Two".to_owned()];

    println!("{:?}", string_slice);

    let str_slice: &[&str] = &["One", "Two"];

    println!("{:?}", str_slice);

    let number_vector = vec![1, 2, 3, 4, 5];

    print_slice(&number_vector);
    match number_vector.as_slice() {
        [first, .., last] => println!("First {}, Last {}", first, last),
        [single] => println!("Single {}", single),
        [] => println!("Nothing Found"),
        _ => unreachable!("Panic Error"),
    }

    match number_vector.as_slice() {
        [first @ 10..=50, ..] => println!("Numbers {}", first),
        [.., last] if last == &5 => println!("Last element found"),
        [] => println!("Nothing Found"),
        _ => unreachable!("Panic Error Printed"),
    }
}
