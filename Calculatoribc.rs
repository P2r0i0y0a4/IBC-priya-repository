fn main() {
    let num1 = 10;
    let num2 = 5;

    println!("Number 1: {}", num1);
    println!("Number 2: {}", num2);

    let sum = add(&num1, &num2);
    let difference = subtract(&num1, &num2);
    let product = multiply(&num1, &num2);
    let quotient = divide(&num1, &num2);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}

fn add(a: &i32, b: &i32) -> i32 {
    a + b
}

fn subtract(a: &i32, b: &i32) -> i32 {
    a - b
}

fn multiply(a: &i32, b: &i32) -> i32 {
    a * b
}

fn divide(a: &i32, b: &i32) -> i32 {
    a / b
}
