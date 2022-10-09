fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6];

    let modified_arr: Vec<_> = arr.iter().map(|num| num + 1).collect();

    println!("{:?}", modified_arr);

    let filtered_arr: Vec<_> = modified_arr.iter().filter(|&num| num > &4).collect();

    println!("{:?}", filtered_arr);

    let last: Option<&i32> = arr.iter().last();

    println!("{:?}", last);

    let find: Option<&i32> = arr.iter().find(|&num| num == &100);

    println!("{:?}", find);

    let min: Option<&i32> = arr.iter().min();

    println!("{:?}", min);

    let max: Option<&i32> = arr.iter().max();

    println!("{:?}", max);

    let count = arr.iter().count();

    println!("{:?}", count);
}
