use std::fmt::Debug;

// marker traits
trait Color {}
trait Variant {}

#[derive(Debug)]
struct Red {}
impl Color for Red {}

#[derive(Debug)]
struct Bike {}
impl Variant for Bike {}

#[derive(Debug)]
struct Vechile<T: Variant, C: Color> {
    variant: T,
    color: C,
}

#[derive(Debug)]
struct VechileAlternative<T, C>
where
    T: Variant,
    C: Color,
{
    variant: T,
    color: C,
}

impl<T, C> Vechile<T, C>
where
    T: Variant + Debug,
    C: Color + Debug,
{
    fn new(variant: T, color: C) -> Self {
        Self { variant, color }
    }

    fn print_vechile(&self) {
        println!("{:?}", self);
    }
}

fn print_vechile_generic_function<T, C>(vechile: Vechile<T, C>)
where
    T: Variant + Debug,
    C: Color + Debug,
{
    println!("{:?}", vechile);
}

fn main() {
    let bike = Vechile {
        variant: Bike {},
        color: Red {},
    };

    bike.print_vechile();
    print_vechile_generic_function(bike);
}
