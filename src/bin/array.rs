fn main() {
    let string_array: [String; 2] = ["One".to_owned(), "Two".to_owned()];

    let mut number_array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    number_array[number_array.len() - 1] = 0;

    println!("{:?}", string_array);
    println!("{:?}", number_array);
}
