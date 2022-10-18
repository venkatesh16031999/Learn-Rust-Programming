#[derive(PartialEq)]
enum AnimalType {
    Bird,
    Pet,
    Fish,
}

struct Animal {
    name: String,
    variant: AnimalType,
}

struct Bird<'a> {
    birds: Vec<&'a Animal>,
}

impl Animal {
    fn list_animals() -> Vec<Self> {
        vec![
            Self {
                name: "Cat".to_owned(),
                variant: AnimalType::Pet,
            },
            Self {
                name: "Dog".to_owned(),
                variant: AnimalType::Pet,
            },
            Self {
                name: "Parrot".to_owned(),
                variant: AnimalType::Bird,
            },
            Self {
                name: "Peacock".to_owned(),
                variant: AnimalType::Bird,
            },
            Self {
                name: "Golden Fish".to_owned(),
                variant: AnimalType::Fish,
            },
            Self {
                name: "Shark".to_owned(),
                variant: AnimalType::Fish,
            },
            Self {
                name: "Angel Fish".to_owned(),
                variant: AnimalType::Fish,
            },
        ]
    }
}

fn main() {
    let animals: Vec<_> = Animal::list_animals();

    let bird = Bird {
        birds: animals
            .iter()
            .filter(|animal| animal.variant == AnimalType::Bird)
            .collect(),
    };

    for bird in bird.birds {
        println!("{:?}", bird.name);
    }
}
