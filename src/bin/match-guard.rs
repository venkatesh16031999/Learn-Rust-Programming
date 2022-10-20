struct Person {
    age: i32,
    name: String,
}

#[derive(PartialEq)]
enum Sports {
    Cricket,
    Football,
    Basketball,
    Tennis,
    Volleyball,
    Score(i32),
}

fn main() {
    use Sports::*;
    let persons: Vec<Person> = vec![
        Person {
            age: 100,
            name: "Person 1".to_owned(),
        },
        Person {
            age: 4,
            name: "Person 2".to_owned(),
        },
        Person {
            age: 40,
            name: "Person 3".to_owned(),
        },
        Person {
            age: 50,
            name: "Person 4".to_owned(),
        },
    ];

    let sport = Football;

    let number = 5;

    for person in persons {
        match person {
            Person {
                age: s @ 40 | s @ 50,
                ..
            } => println!("Age includes {}", s),
            Person { age, .. } if age == 4 => println!("Age is 4"),
            Person {
                age: s @ 100..=200, ..
            } => println!("Age is more than 99"),
            Person { .. } => println!("Failed to parse"),
        }
    }

    match sport {
        Tennis | Basketball => println!("Boring games..."),
        Football => println!("My favourite"),
        s if s == Cricket => println!("I like cricket also"),
        Score(s @ 10) => println!("score is 10"),
        _ => println!("Others.."),
    }

    match number {
        n @ 5 | n @ 10 => println!("Number is 5 / 10"),
        n @ 100..=200 => println!("Number is between 100 to 200"),
        n => println!("Number is {}", n),
    }
}
