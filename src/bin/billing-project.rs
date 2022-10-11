use std::io;

enum MainMenu {
    AddBill,
    RemoveBill,
    ViewBill,
    UpdateBill,
    Exit,
}

impl MainMenu {
    fn show_options() {
        println!("Billing Main Menu:");
        println!("1. Add Bill");
        println!("2. Remove Bill");
        println!("3. View Bill");
        println!("4. Update Bill");
        println!("5. Exit");
        println!("Select the menu option:");
    }

    fn from_str(option: &str) -> Option<Self> {
        match option {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::RemoveBill),
            "3" => Some(Self::ViewBill),
            "4" => Some(Self::UpdateBill),
            "5" => Some(Self::Exit),
            _ => None,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Invalid input");
    }
    buffer = buffer.trim().to_owned();
    if buffer == "" {
        None
    } else {
        Some(buffer)
    }
}

fn main() {
    loop {
        MainMenu::show_options();
        let mut input = get_input();

        match input {
            Some(option) => {
                match MainMenu::from_str(option.as_str()) {
                    Some(MainMenu::AddBill) => println!("1"),
                    Some(MainMenu::RemoveBill) => println!("2"),
                    Some(MainMenu::ViewBill) => println!("3"),
                    Some(MainMenu::UpdateBill) => println!("4"),
                    Some(MainMenu::Exit) => {
                        println!("Thank you...");
                        break;
                    }
                    None => println!("Option not available, please try again!"),
                };
            }
            None => println!("Invalid input"),
        }
    }
}
