fn main(){
    let numbers_one = vec![1,2,3,4];

    for num in numbers_one {
        println!("{:?}", num);
    }

    let mut numbers_two = vec![1,2,3,4];

    numbers_two.push(5);

    for num in numbers_two {
        println!("{:?}", num);
    }

    let mut numbers_three = Vec::new();

    numbers_three.push(1);
    numbers_three.push(2);
    numbers_three.push(3);
    numbers_three.push(4);
    numbers_three.push(5);

    println!("Length before pop() : {:?}", numbers_three.len());

    numbers_three.pop();

    println!("Length after pop() : {:?}", numbers_three.len());

    for num in &numbers_three {
        println!("{:?}", num);
    }

    println!("{:?}", numbers_three[1]);
}