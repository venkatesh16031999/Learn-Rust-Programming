struct Employee {
    age: i32,
    experience: i32,
}

fn is_eligible(employee: &Employee) -> Result<bool, String> {
    match employee {
        Employee { age, experience } => {
            if age <= &10 {
                return Err("Employee is under aged".to_owned());
            }

            if experience > &1 {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
    }
}

fn is_eligible_by_operator(employee: &Employee) -> Result<(), String> {
    let eligible = is_eligible(employee)?;
    print_employee_stage(&eligible);
    Ok(())
}

fn print_employee_stage(is_eligible: &bool) {
    match is_eligible {
        true => println!("Exprienced"),
        false => println!("Fresher")
    }
}

fn main() {
    let employees = vec![
        Employee{age: 10, experience: 0},
        Employee{age: 29, experience: 8},
    ];

    for employee in employees {
        let eligibility = is_eligible(&employee);

        match &eligibility {
            Err(error) => println!("Error: {:?}", error),
            Ok(eligible) => {
                print_employee_stage(&eligible);
            },
        }

        let eligibility_by_operator = is_eligible_by_operator(&employee);

        match &eligibility_by_operator {
            Err(error) => println!("Error: {:?}", error),
            _ => ()
        }
    }
}