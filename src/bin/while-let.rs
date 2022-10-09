fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut iterator = arr.iter();
    while let Some(num) = iterator.next() {
        println!("{:?}", num);
    }

    for num in arr.iter() {
        println!("{:?}", num);
    }
}
