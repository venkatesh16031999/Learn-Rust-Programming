trait Movement {
    fn move_action(&self);
}

struct Animal {
    name: String,
    movement: String,
}

impl Movement for Animal {
    fn move_action(&self) {
        println!("{:?} is {:?}", self.name, self.movement);
    }
}

fn print_movement(animals: Vec<Box<dyn Movement>>) {
    for animal in animals {
        animal.move_action();
    }
}

fn print_movement_two(animals: Vec<&dyn Movement>) {
    for animal in animals {
        animal.move_action();
    }
}

fn main() {
    let bird: Box<dyn Movement> = Box::new(Animal {
        name: "Bird".to_owned(),
        movement: "Flying".to_owned(),
    });
    let fish: Box<dyn Movement> = Box::new(Animal {
        name: "Fish".to_owned(),
        movement: "Swiming".to_owned(),
    });

    let animals = vec![bird, fish];

    print_movement(animals);

    let bird_one = Box::new(Animal {
        name: "Bird".to_owned(),
        movement: "Flying".to_owned(),
    });
    let fish_one = Box::new(Animal {
        name: "Fish".to_owned(),
        movement: "Swiming".to_owned(),
    });

    let animals_one: Vec<Box<dyn Movement>> = vec![bird_one, fish_one];

    print_movement(animals_one);

    let bird_two: &dyn Movement = &Animal {
        name: "Bird".to_owned(),
        movement: "Flying".to_owned(),
    };
    let fish_two: &dyn Movement = &Animal {
        name: "Fish".to_owned(),
        movement: "Swiming".to_owned(),
    };

    let animals_two: Vec<&dyn Movement> = vec![bird_two, fish_two];

    print_movement_two(animals_two);
}
