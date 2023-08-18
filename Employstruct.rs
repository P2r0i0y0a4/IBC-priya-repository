use std::io;

// Define the Employee struct
struct Employee {
    employee_name: String,
    employee_id: u32,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    fn new() -> Employee {
        Employee {
            employee_name: String::new(),
            employee_id: 0,
            email: String::new(),
            age: 0,
            phone_number: String::new(),
        }
    }
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    // Input employee data
    let mut employee = Employee::new();
    println!("Enter employee details:");
    println!("Name: ");
    io::stdin().read_line(&mut employee.employee_name).expect("Failed to read input");
    println!("Employee ID: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    employee.employee_id = input.trim().parse().expect("Invalid input");
    println!("Email: ");
    io::stdin().read_line(&mut employee.email).expect("Failed to read input");
    println!("Age: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    employee.age = input.trim().parse().expect("Invalid input");
    println!("Phone Number: ");
    io::stdin().read_line(&mut employee.phone_number).expect("Failed to read input");

    employees.push(employee);

    // Call functions to retrieve employee details
    let id_to_search = 1; // Change this to the desired employee ID
    if let Some(found_employee) = get_employee_by_id(&employees, id_to_search) {
        println!("Employee found: {:?}", found_employee);
    } else {
        println!("Employee not found with ID: {}", id_to_search);
    }

    let age_to_search = 30; // Change this to the desired age
    let employees_with_age = get_employees_by_age(&employees, age_to_search);
    if employees_with_age.is_empty() {
        println!("No employees found with age: {}", age_to_search);
    } else {
        println!("Employees with age {}: {:?}", age_to_search, employees_with_age);
    }
}

fn get_employee_by_id(employees: &Vec<Employee>, id: u32) -> Option<&Employee> {
    employees.iter().find(|&employee| employee.employee_id == id)
}

fn get_employees_by_age(employees: &Vec<Employee>, age: u32) -> Vec<&Employee> {
    employees.iter().filter(|&employee| employee.age == age).collect()
}
