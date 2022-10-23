fn operation_function(a: i32, b: i32, operation: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    operation(a, b)
}

fn main() {
    let add = Box::new(|a, b| a + b);

    let result = operation_function(10, 20, add);
    println!("Addition Result: {:?}", result);

    let sub = Box::new(|a, b| a - b);

    let result = operation_function(20, 10, sub);
    println!("Subtraction Result: {:?}", result);
    let mul = Box::new(|a, b| a * b);

    let result = operation_function(10, 20, mul);
    println!("Multiplication Result: {:?}", result);

    let operation_name = "Division";

    // moving the outer variables into the cloisure
    let div = Box::new(move |a, b| {
        print!("{} Result:", operation_name);
        a / b
    });

    let result = operation_function(20, 10, div);
    println!("{:?}", result);
}
