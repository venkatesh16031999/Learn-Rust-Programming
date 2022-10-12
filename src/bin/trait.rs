trait Animal {
    fn move_action(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn move_action(&self) {
        println!("{:?} is running", self.name);
    }
}

struct Shark {
    name: String,
}

impl Animal for Shark {
    fn move_action(&self) {
        println!("{:?} is running", self.name);
    }
}

fn move_animal(animal: impl Animal) {
    animal.move_action();
}

fn main() {
    let shark = Shark {
        name: "Shark".to_owned(),
    };
    let dog = Dog {
        name: "Dog".to_owned(),
    };
    move_animal(shark);
    move_animal(dog);
}
