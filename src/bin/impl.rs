struct Temperature {
    degree: i32,
}

impl Temperature{
    fn cold() -> Self {
        Self {
            degree: 32
        }
    }

    fn warm() -> Self {
        Self {
            degree: 70
        }
    }

    fn print_climate(&self) {
        match self.degree {
            32 => println!("Cold"),
            70 => println!("Warm"),
            _ => println!("Other"),
        }
    }
}

fn main() {
    let cold = Temperature::cold();
    let warm = Temperature::warm();

    cold.print_climate();
    warm.print_climate();
}