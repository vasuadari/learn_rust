use std::io;
use std::collections::{BTreeMap, HashMap};

fn menu() {
  println!("
Manage employee database
1. Add employee name to department
2. Retrieve list of people in a department
3. List all people in the company by department, sorted alphabetically
4. Quit");
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>) {
  const ERROR_MSG: &str = "Unable to add employee. Format is invalid.";

  println!("Format: Add Sally to Engineering");

  let mut input_text = String::new();

  io::stdin().read_line(&mut input_text)
    .expect("Failed to read line");

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
    let department = match vec.get(3) {
      Some(string) => *string,
      None => panic!(ERROR_MSG),
    };

    let department_vec = employees.entry(department.to_string())
      .or_insert(Vec::new());
    department_vec.push(employee_name.to_string())
  }
  else {
    panic!(ERROR_MSG);
  }
}

fn employees_by_department(employees: &HashMap<String, Vec<String>>) {
  let mut department_name = String::new();

  println!("Enter Department name:");

  io::stdin().read_line(&mut department_name)
    .expect("Failed to read line");

  let department_name: &str = department_name.trim();

  match employees.get(department_name) {
    Some(vec) => {
      println!("Employees in {} department", department_name);
      for employee_name in vec.iter() {
        println!("{}", employee_name);
      }
    },
    None => println!("No employees in {} department", department_name),
  }
}

fn list_employees(employees: &mut HashMap<String, Vec<String>>) {
  let sorted_employees: BTreeMap<_, _> = employees.iter().collect();

  for (department, _) in sorted_employees.iter() {
    println!("Employees in {} department", department);

    if let Some(employees_vec) = employees.get(*department) {
      let mut employees_vec = employees_vec.to_vec();
      employees_vec.sort();

      for employee_name in employees_vec.iter() {
        println!("{}", employee_name);
      }
    }
  }
}

fn main() {
  let mut employees: HashMap<String, Vec<String>> = HashMap::new();

  loop {
    let mut choice = String::new();

    menu();

    io::stdin().read_line(&mut choice).
        expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
      Ok(num) => num,
      Err(_) => break,
    };

    match choice {
      1 => add_employee(&mut employees),
      2 => employees_by_department(&employees),
      3 => list_employees(&mut employees),
      _ => break,
    }
  }
}
