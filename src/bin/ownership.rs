struct Person {
    age: i32,
    lucky_number: i32,
}

fn print_person(person: &Person) {
    println!("{:?} {:?}", person.age, person.lucky_number);
}

fn main() {
    let person_one = Person{
        age: 23,
        lucky_number: 4
    };

    // memory - move 
    // print_person(&person_one);
    // print_person(&person_one);

    // memory - borrow
    print_person(&person_one);
    print_person(&person_one);
}