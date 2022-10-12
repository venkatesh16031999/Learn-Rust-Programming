struct Animal {
    name: String,
}

impl Default for Animal {
    fn default() -> Self {
        Self {
            name: "Default Animal".to_owned(),
        }
    }
}

fn main() {
    let animal = Animal::default();

    println!("{:?}", animal.name);
}
