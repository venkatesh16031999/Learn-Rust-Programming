struct StringHolder {
    string_temp: String,
}

fn main() {
    let str_one = "borrowed by default";

    println!("{:?}", str_one);

    let str_two = "to-owner() original string".to_owned();

    println!("{:?}", str_two);

    let str_three = String::from(" String::from original string");

    println!("{:?}", str_three);

    let str_four = StringHolder{ string_temp: str_three };

    println!("{:?}", str_four.string_temp);
}