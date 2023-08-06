enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
fn main() {
    // Prompt the user to input the first number, the operation to be performed, and the second number.
    println!("Enter the first number:");
    let mut first_number = String::new();
    std::io::stdin().read_line(&mut first_number).expect("Failed to read input.");
    let first_number: f64 = first_number.trim().parse().expect("Invalid input. Please enter a valid number.");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).expect("Failed to read input.");
    let operation: char = operation.trim().chars().next().expect("Invalid input. Please enter a valid operation.");

    println!("Enter the second number:");
    let mut second_number = String::new();
    std::io::stdin().read_line(&mut second_number).expect("Failed to read input.");
    let second_number: f64 = second_number.trim().parse().expect("Invalid input. Please enter a valid number.");

    // Create an Operation enum instance with the parsed input values.
    let op_enum = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation. Please choose from +, -, *, /.");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance.
    let result = calculate(op_enum);

    // Print the result to the console.
    println!("Result: {}", result);
}