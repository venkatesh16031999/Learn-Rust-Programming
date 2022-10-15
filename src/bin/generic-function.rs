trait Color {
    fn display_color(&self);
}

struct Dress {
    name: String,
    color: String,
}

impl Color for Dress {
    fn display_color(&self) {
        println!("{:?}", self.color);
    }
}

fn generic_function_1(dress: impl Color) {
    dress.display_color();
}

fn generic_function_2<T: Color>(dress: T) {
    dress.display_color();
}

fn generic_function_3<T>(dress: T)
where
    T: Color,
{
    dress.display_color();
}

fn main() {
    let dress_one = Dress {
        name: "T shirt".to_owned(),
        color: "Black".to_owned(),
    };

    generic_function_1(dress_one);

    let dress_two = Dress {
        name: "T shirt".to_owned(),
        color: "Blue".to_owned(),
    };
    generic_function_2(dress_two);

    let dress_three = Dress {
        name: "T shirt".to_owned(),
        color: "White".to_owned(),
    };
    generic_function_3(dress_three);
}
