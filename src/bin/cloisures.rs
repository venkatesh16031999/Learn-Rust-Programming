fn main() {
    let cloisure_full_function = |a: i32, b: i32| -> i32 {
        a + b
    };

    println!("Sum One: {:?}", cloisure_full_function(1,2));

    let cloisure_short_function = |a, b| a + b;

    println!("Sum Two: {:?}", cloisure_short_function(1,2));
}