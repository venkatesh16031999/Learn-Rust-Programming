fn main() {
    let some_value = Some(10);

    let none_value = None;

    println!("{:?}", some_value.is_some());

    println!("{:?}", some_value.is_none());

    println!("{:?}", some_value.filter(|num| num > &10));

    println!("{:?}", some_value.map(|num| num + 1));

    println!("{:?}", none_value.or_else(|| Some(0)));

    println!("{:?}", some_value.unwrap_or_else(|| 0));
}