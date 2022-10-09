fn main() {
    let some_value = Some(11);

    if let Some(num) = some_value {
        println!("{:?}", some_value);
    }

    let none_value: Option<i32> = None;

    if let Some(num) = none_value {
        println!("{:?}", none_value);
    } else {
        println!("Nothing found");
    }
}
