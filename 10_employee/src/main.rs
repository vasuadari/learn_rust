use std::collections::{BTreeMap, HashMap};
use std::io;

fn menu() {
    println!(
        "
Manage employee database
1. Add employee name to department
2. Retrieve list of people in a department
3. List all people in the company by department, sorted alphabetically
4. Quit"
    );
}

fn get_input(title: &str) -> String {
    println!("{}", title);
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    input_text
}

fn add_employee(
    employees: &mut HashMap<String, Vec<String>>,
    employee_name: &str,
    department_name: &str,
) {
    employees
        .entry(department_name.to_string())
        .or_insert(Vec::new())
        .push(employee_name.to_string());
}

fn employees_by_department(employees: &HashMap<String, Vec<String>>, department_name: &str) {
    match employees.get(department_name) {
        Some(vec) => {
            println!("Employees in {} department", department_name);
            for employee_name in vec.iter() {
                println!("{}", employee_name);
            }
        }
        None => println!("No department named {}", department_name),
    }
}

fn list_employees(employees: &mut HashMap<String, Vec<String>>) {
    let sorted_employees: BTreeMap<_, _> = employees.iter().collect();

    for (department, _) in sorted_employees.iter() {
        employees_by_department(employees, department);
    }
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut choice = String::new();

        menu();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        match choice {
            1 => {
                const ERROR_MSG: &str = "Unable to add employee. Format is invalid.";
                let input_text = get_input("Format: Add Sally to Engineering");
                let vec: Vec<&str> = input_text.split_whitespace().collect();

                if vec.len() == 4 {
                    match vec[0] == "Add" {
                        true => (),
                        _ => panic!(ERROR_MSG),
                    }

                    match vec[2] == "to" {
                        true => (),
                        _ => panic!(ERROR_MSG),
                    }

                    let employee_name = match vec.get(1) {
                        Some(string) => *string,
                        None => panic!(ERROR_MSG),
                    };
                    let department_name = match vec.get(3) {
                        Some(string) => *string,
                        None => panic!(ERROR_MSG),
                    };

                    add_employee(&mut employees, employee_name, department_name)
                } else {
                    panic!(ERROR_MSG);
                }
            }
            2 => {
                let department_name = get_input("Enter Department name:");
                let department_name: &str = department_name.trim();

                employees_by_department(&employees, department_name)
            }
            3 => list_employees(&mut employees),
            _ => break,
        }
    }
}
