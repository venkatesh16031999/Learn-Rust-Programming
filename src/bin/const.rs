const MAX_NUMBER: i32 = 100;

fn clamp(num: i32) -> i32 {
    if num > MAX_NUMBER {
        MAX_NUMBER
    } else {
        num
    }
}

fn main() {
    let a = 10;
    let b = 70;

    let c = a + b;

    let ans = clamp(c);

    println!("{:?}", ans);
}
