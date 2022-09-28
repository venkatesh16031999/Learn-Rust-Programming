fn tuple_return_function(a: i32, b: i32, c: i32) -> (i32, i32) {
    (a + b, b + c)
}

fn main() {
    let (x, y) = ("tuple_one", "tuple_two");

    println!("{:?} {:?}", x, y);

    let (n1, n2) = tuple_return_function(1,2,3);

    println!("{:?} {:?}", n1, n2);
}