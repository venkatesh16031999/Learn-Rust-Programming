use std::thread;
use std::time::Duration;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let thread_one = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("1 second delayed thread")
    });

    thread_one.join();

    let n1 = 10;
    let n2 = 20;

    let thread_two = thread::spawn(move || add(n1, n2));

    let result = thread_two.join();

    if let Ok(res) = result {
        println!("Result: {:?}", res);
    } else {
        println!("Failed");
    }
}
